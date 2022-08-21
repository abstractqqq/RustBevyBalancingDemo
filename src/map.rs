use std::{collections::HashMap, ops::RangeInclusive};
use itertools::{Itertools, iproduct};
use bevy::{prelude::*};
use bevy_inspector_egui::Inspectable;
use rand::{Rng};
use crate::player::{State, Player};
use crate::ascii::{AsciiSheet, spawn_ascii_sprite};

pub const TILE_SIZE:f32 = 32.0;
pub const WORLD_X:[i32;2] = [-50,50]; // element at 0 must be smaller than element at 1
pub const WORLD_Y:[i32;2] = [-50,50]; 
pub const GROUP_SIZE:u8 = 3; // must be >= 3
pub const COLORS:[Color;GROUP_SIZE as usize] = [Color::SILVER, Color::GOLD, Color::BLUE];
const THRESHOLD:f32 = 1500.0;

pub struct Instruction {
    pub transfer_point: (usize, usize),
    pub to_group: u8
}

#[derive(Component)]
pub struct Tile {
    pub loc: (usize, usize)
}

#[derive(Component, Inspectable)]
pub struct MapData {
    #[inspectable(ignore)]
    pub tile_groups : Vec<Vec<u8>>,
    #[inspectable(ignore)]
    tile_values : Vec<Vec<u32>>,
    group_sum: [u32; GROUP_SIZE as usize],
    pub group_balance: f32,
    #[inspectable(ignore)]
    pub tile_neighbors: HashMap<(usize, usize), Vec<(usize, usize)>>,
    #[inspectable(ignore)]
    interval: f32
}
impl MapData {

    fn update_balance(&mut self) {
        self.group_balance = MapData::calc_balance(&self.group_sum);
    }

    fn calc_balance(data:&[u32; GROUP_SIZE as usize]) -> f32 {
        let mut s:i32 = 0;
        let mut counter: f32 = 0.0;
        for (i, j) in (0..data.len()).tuple_combinations::<(usize, usize)>() {
            s += (data[i] as i32 - data[j] as i32).abs();
            counter += 1.0;
        }
        s as f32 / counter
    }

    fn calculate_neighbors(&mut self, radius: usize) {
        let x_range = (WORLD_X[1]-WORLD_X[0]) as usize;
        let y_range = (WORLD_Y[1]-WORLD_Y[0]) as usize;
        for (x,y) in iproduct!(1..x_range, 1..y_range){
            let group:u8 = self.tile_groups[x][y];
            if group == GROUP_SIZE {
                continue
            }
            for (xx,yy) in MapData::get_nbhd((x,y), radius) {
                if  self.tile_groups[xx][yy] < GROUP_SIZE && self.tile_groups[xx][yy] != group {
                    let vec = self.tile_neighbors.get_mut(&(x,y)).unwrap();
                    vec.push((xx,yy));
                }
            } // same code used twice.
        }
    }

    pub fn reassign_group(&mut self, point:(usize, usize), to_group:u8, radius: usize) {
        let v = self.tile_values[point.0][point.1];
        let from_group = self.tile_groups[point.0][point.1];
        self.group_sum[from_group as usize] -= v;
        self.group_sum[to_group as usize] += v;
        self.tile_groups[point.0][point.1] = to_group;
        self.update_balance();
        let vec = self.tile_neighbors.get_mut(&point).unwrap();
        vec.clear();
        for (xx, yy) in MapData::get_nbhd(point, radius) {
            if  self.tile_groups[xx][yy] < GROUP_SIZE && self.tile_groups[xx][yy] != to_group {
                vec.push((xx,yy));
            }
        }
    }

    fn find_min_group(&self) -> usize {
        let mut min_idx: usize = 0;
        let mut min_val = u32::MAX;
        for i in 0..self.group_sum.len() {
            if self.group_sum[i] < min_val {
                min_val = self.group_sum[i];
                min_idx = i;
            }
        }
        min_idx
    }

    fn create_player_instruction(&self, group:usize) -> Option<Instruction>{
        for (key, nbhd) in self.tile_neighbors.iter(){
            if !nbhd.is_empty() && self.tile_groups[key.0][key.1] == group as u8 {
                // After adding self.tile_groups[key.0][key.1] == group as u8
                // nothing worked anymore... ...
                // println!("Here");
                for candidate in nbhd.iter() {
                    let x = candidate.0;
                    let y = candidate.1;
                    let giver:u8 = self.tile_groups[x][y];
                    let taker:u8 = self.tile_groups[key.0][key.1];
                    let test_point = (x,y);
                    if self.should_take(test_point, giver, taker) {
                        return Some(Instruction {transfer_point: test_point, to_group: taker})
                    }
                }
            } 
        }
        None
    }

    fn should_take(&self, giver_point:(usize, usize), giver_group:u8, taker_group:u8,) -> bool {
        let mut temp_group_sum:[u32; GROUP_SIZE as usize] = self.group_sum.clone();
        let value_transfered:u32 = self.tile_values[giver_point.0][giver_point.1];

        temp_group_sum[giver_group as usize] -= value_transfered;
        temp_group_sum[taker_group as usize] += value_transfered;

        let new_balance = MapData::calc_balance(&temp_group_sum);

        new_balance < self.group_balance

    }

    fn get_nbhd(point:(usize, usize), radius:usize) 
        -> itertools::Product<RangeInclusive<usize>,RangeInclusive<usize>> 
    {
        let x_range = (WORLD_X[1]-WORLD_X[0]) as usize;
        let y_range = (WORLD_Y[1]-WORLD_Y[0]) as usize;
        let x = point.0;
        let y = point.1;
        let x_min = 0.max(x as i32 - radius as i32) as usize;
        let x_max = (x_range as i32).min(x as i32 + radius as i32) as usize;
        let y_min = 0.max(y as i32 - radius as i32) as usize;
        let y_max = (y_range as i32).min(y as i32 + radius as i32) as usize;

        iproduct!(x_min..=x_max, y_min..=y_max)

    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app:&mut App){
        app.add_startup_system(generate_map)
        .add_system(start_algorithm.label("algorithm"));
    }
}

fn generate_map(mut commands: Commands, ascii:Res<AsciiSheet>, asset_server: Res<AssetServer>){
    let mut tiles: Vec<Entity> = Vec::new();
    let mut tile_groups: Vec<Vec<u8>> = Vec::new();
    let mut tile_values: Vec<Vec<u32>> = Vec::new();
    let mut tile_neighbors: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut group_sums: [u32; GROUP_SIZE as usize] = [0; GROUP_SIZE as usize];
    // const OFFSET:Vec2 = Vec2::new(30.,-15.);
    for x in WORLD_X[0]..=WORLD_X[1] {
        let mut row_tile_groups:Vec<u8>= Vec::new();
        let mut row_tile_values:Vec<u32>= Vec::new();
        for y in WORLD_Y[0]..=WORLD_Y[1]{
            let x_idx = (x - WORLD_X[0]) as usize;
            let y_idx = (y - WORLD_Y[0]) as usize;
            tile_neighbors.insert((x_idx, y_idx), Vec::new());
            if y == WORLD_Y[0] || y == WORLD_Y[1] || x == WORLD_X[0] || x == WORLD_X[1] {
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    '#' as usize,
                    Color::rgb(0.9,0.9,0.9),
                    Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 100.0),
                    (x_idx, y_idx)
                );
                tiles.push(tile);
                row_tile_groups.push(GROUP_SIZE);
                row_tile_values.push(0);
            } else {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0..100) < 33 {
                    let mut group: u8 = 0;
                    if x < 0 && y <= 0 {
                        group = 1;
                    } else if x >= 0 && y <= 0 {
                        group = 2;
                    }
                    let value:u32 = rng.gen_range(1..=(group as u32 + 1)*10);
                    row_tile_groups.push(group);
                    row_tile_values.push(value);
                    group_sums[group as usize] += value;
                    let mut c = COLORS[group as usize];
                    c.set_a(0.5);
                    let num_tile = spawn_ascii_sprite(
                        &mut commands,
                        &ascii,
                        43,
                        c,
                        Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 100.0),
                        (x_idx, y_idx)
                    );
                    
                    let text_child = commands.spawn_bundle(Text2dBundle {
                        text: Text::from_section(value.to_string(),
                        TextStyle {
                                     font: asset_server.load("fonts/ArchitectsDaughter-Regular.ttf"),
                                     font_size: 20.0,
                                     color: Color::WHITE,
                                }
                        )
                        , transform: Transform::from_translation(Vec3::new(-15.0,25.0,0.0))
                        , ..Default::default()
                        
                    }).id();
                    commands.entity(num_tile).push_children(&[text_child]);
                    tiles.push(num_tile);
                } else{
                    row_tile_groups.push(GROUP_SIZE);
                    row_tile_values.push(0);
                }
            }
        }
        tile_groups.push(row_tile_groups);
        tile_values.push(row_tile_values);
    }
    
    let mut map = MapData {tile_groups: tile_groups, tile_values: tile_values
        , group_sum: group_sums, tile_neighbors: tile_neighbors, group_balance: 0.0, interval: 0.0};
    map.calculate_neighbors(2);
    map.update_balance();
    commands.spawn_bundle(VisibilityBundle::default())
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(map)
        .push_children(&tiles);

}

fn start_algorithm(mut query: Query<&mut Player>
    , mut map_query: Query<&mut MapData>
    , mut tile_query: Query<(&Tile, &mut TextureAtlasSprite), (With<Tile>, Without<Player>)>
    , t: Res<Time>
){
    let mut map_data = map_query.single_mut();
    let mut player= query.single_mut();
    if map_data.group_balance < THRESHOLD {
        println!("Optimization Completed.");
        return ()
    }
    map_data.interval += t.delta_seconds();
    if map_data.interval > 0.05 {
        map_data.interval -= 0.05;
        match player.state {
            State::Idle => {
                let start_group = map_data.find_min_group();
                let ins = map_data.create_player_instruction(start_group);
                match ins {
                    Some(i) => {
                        let grid_pos = i.transfer_point;
                        for (tile, mut tile_sprite) in tile_query.iter_mut() {
                            if tile.loc == grid_pos {
                                tile_sprite.color = Color::RED;
                                break;
                            }
                        }
                        println!("Instruction sent to player. Moving to point {:?}", i.transfer_point);
                        player.queue_instruction(i);
                    },
                    _ => {println!("Something wrong happened.");}
                }
    
            },
            _ => {}
    
        }
    }


}