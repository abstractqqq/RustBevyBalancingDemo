use bevy::{prelude::*};
use rand::{Rng, thread_rng, seq::SliceRandom};
use bevy_inspector_egui::Inspectable;
use crate::{ascii::{spawn_ascii_sprite, AsciiSheet}, map::{TILE_SIZE, WORLD_X, WORLD_Y, COLORS, MapData, Instruction, Tile}};
use bevy_easings::*;
use std::collections::VecDeque;

pub enum State {
    Idle,
    Moving
}

#[derive(Clone, Copy)]
enum Movement {
    Up,
    Down,
    Left,
    Right
}

impl Movement {
    fn v(&self) -> Vec2 {
        match *self {
            Movement::Up => Vec2::new(0.0, 1.0),
            Movement::Down => Vec2::new(0.0, -1.0),
            Movement::Left => Vec2::new(-1.0, 0.0),
            Movement::Right => Vec2::new(1.0, 0.0)
        }
    }
    // fn random() -> Movement {
    //     let mut rng = rand::thread_rng();
    //     match rng.gen_range(0..4) {
    //         0 => Movement::Up,
    //         1 => Movement::Down,
    //         2 => Movement::Left,
    //         _ => Movement::Right
    //     }
    // }
}

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    #[inspectable(ignore)]
    pub state: State,
    interval: f32,
    #[inspectable(ignore)]
    move_queue: Vec<Movement>,
    #[inspectable(ignore)]
    instruction_queue:VecDeque<Instruction>

}

impl Player {
    pub fn queue_instruction(&mut self, instruction:Instruction){
        self.instruction_queue.push_back(instruction);
    }

    fn route_planning(current_pos: Vec3, target: Vec3) -> Vec<Movement> {

        let diff = (target - current_pos) / TILE_SIZE;
        let x_diff = diff.x as i32;
        let y_diff = diff.y as i32;
        let x_abs = x_diff.abs() as u32;
        let y_abs = y_diff.abs() as u32;
        let sum = x_abs + y_abs;
    
        let x_move = if x_diff < 0 {
            Movement::Left
        } else {
            Movement::Right
        };
    
        let y_move = if y_diff < 0 {
            Movement::Down
        } else {
            Movement::Up
        };
        
        let mut random_vec:Vec<u32> = (0..sum).collect();
        random_vec.shuffle(&mut thread_rng());
        random_vec.iter().map(|x| {
            if x < &x_abs {
                x_move
            } else {
                y_move
            }
        }).collect()
    
    } 

    fn start_moving(&mut self, player_pos:Vec3){
        let job = self.instruction_queue.front();
        match job {
            Some(j) => {
                let x:f32 = (j.transfer_point.0 as f32 + WORLD_X[0] as f32) * TILE_SIZE;
                let y:f32 = (j.transfer_point.1 as f32 + WORLD_Y[0] as f32) * TILE_SIZE;
                let target:Vec3 = Vec3::new(x, y, player_pos.z);
                self.move_queue.append(&mut Player::route_planning(player_pos, target));
                self.state = State::Moving;
            },
            _ => {}
        }
    }

}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app:&mut App) {
        app.add_startup_system(spawn_player)
        .add_system(player_control.label("movement"))
        .add_system(camera_follow.after("movement"));
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>){
    let player = spawn_ascii_sprite(&mut commands, &ascii, 8, Color::rgb(1.,0.,0.), Vec3::new(0.0,0.0,900.0), (0,0));
    // let menu = spawn_menu(&mut commands, asset_server);
    // let route = route_planning(Vec3::new(0., 0., 0.), Vec3::new(160., 320., 0.));

    commands.entity(player)
        .insert(Name::new("Player"))
        .insert(Player {speed: TILE_SIZE, state: State::Idle, interval: 0.0
            , move_queue: Vec::new(), instruction_queue: VecDeque::new()});

}

fn player_control(mut query: Query<(&mut Player, &mut TextureAtlasSprite, &mut Transform), With<Player>>
    // , keyboard: Res<Input<KeyCode>>
    , mut map_query: Query<&mut MapData>
    , mut tile_query: Query<(&Tile, &mut TextureAtlasSprite), (With<Tile>, Without<Player>)>
    , t: Res<Time>
){
    let (mut p, mut sprite, mut transform) = query.single_mut();
    let mut map_data = map_query.single_mut();
    // (std::f32::consts::PI * 0.75 * t.seconds_since_startup() as f32).sin().abs()
    sprite.color.set_a((std::f32::consts::PI * 0.8 * t.seconds_since_startup() as f32).sin().abs());
    let current_pos = transform.translation;
    
    // if keyboard.just_pressed(KeyCode::C){
    //     let x = ((transform.translation.x / TILE_SIZE) - WORLD_X[0] as f32) as usize;
    //     let y = ((transform.translation.y / TILE_SIZE) - WORLD_Y[0] as f32) as usize;
    //     let neighbors = map_data.tile_neighbors.get(&(x,y)).unwrap();
    //     println!("Self is {:?}", (x,y));
    //     println!("Self group is {:?}", map_data.tile_groups[x][y]);
    //     println!("{:?}", neighbors);
    // }

    match p.state {
        State::Moving => {
            p.interval += t.delta_seconds();
            if p.interval > 0.05 {
                p.interval -= 0.05;
                if p.move_queue.is_empty(){
                    if let Some(instr) = p.instruction_queue.pop_front(){
                        // empty out instruction when it's done.
                        map_data.reassign_group(instr.transfer_point, instr.to_group, 2);
                        for (tile, mut tile_sprite) in tile_query.iter_mut() {
                            if tile.loc == instr.transfer_point {
                                tile_sprite.color = COLORS[instr.to_group as usize];
                                break;
                            }
                        }
                        p.interval = 0.0;
                        p.state = State::Idle;
                    }
                } else {
                    let step = p.move_queue.pop().unwrap();
                    transform.translation += get_move_delta(current_pos, step, 1.0);
                }
            }
        },
        State::Idle => {
            if !p.instruction_queue.is_empty(){
                p.start_moving(transform.translation);
            }
        }
    }

    // if keyboard.just_pressed(KeyCode::Up){
    //     transform.translation += get_move_delta(current_pos, Movement::Up, multiplier);
    // }
    // else if keyboard.just_pressed(KeyCode::Down){
    //     transform.translation += get_move_delta(current_pos, Movement::Down, multiplier);
    // }
    // else if keyboard.just_pressed(KeyCode::Left){
    //     transform.translation += get_move_delta(current_pos, Movement::Left, multiplier);
    // }
    // else if keyboard.just_pressed(KeyCode::Right){
    //     transform.translation += get_move_delta(current_pos, Movement::Right, multiplier);
    // }

}

fn get_move_delta(current_pos: Vec3, dir: Movement, length: f32) -> Vec3{
    let temp = dir.v();
    let pos = Vec3::new(temp.x, temp.y, 0.0);
    let target = current_pos + length * pos * TILE_SIZE;
    overshoot_check(target, current_pos)
}

fn overshoot_check(target: Vec3, player: Vec3) -> Vec3 {

    if target.x <= WORLD_X[0] as f32 * TILE_SIZE {
        return Vec3::new((WORLD_X[0]+1) as f32 * TILE_SIZE - player.x, 0., 0.)
    }
    if WORLD_X[1] as f32 * TILE_SIZE <= target.x {
        return Vec3::new((WORLD_X[1]-1) as f32 * TILE_SIZE - player.x , 0., 0.)
    }
    if target.y <= WORLD_Y[0] as f32 * TILE_SIZE {
        return Vec3::new(0., (WORLD_Y[0]+1) as f32 * TILE_SIZE - player.y, 0.)
    }
    if WORLD_Y[1] as f32 * TILE_SIZE <= target.y {
        return Vec3::new(0., (WORLD_Y[1]-1) as f32 * TILE_SIZE - player.y, 0.)
    }
    target - player
}

fn camera_follow(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(Entity, &Transform), (With<Camera2d>, Without<Player>)>
){
    let transform = player_query.single();
    let (camera, camera_transform) = camera_query.single_mut();
    if (camera_transform.translation.x - transform.translation.x).abs() > 760.0 {
        let target = Vec3::new(transform.translation.x, camera_transform.translation.y, camera_transform.translation.z);
        commands.entity(camera).insert(
            camera_transform.ease_to(
                Transform{
                    translation: target
                    , rotation: camera_transform.rotation
                    , scale: camera_transform.scale
                }
                , bevy_easings::EaseFunction::CubicOut
                , bevy_easings::EasingType:: Once {duration: std::time::Duration::from_millis(400)}
            )
        );
    }
    else if (camera_transform.translation.y - transform.translation.y).abs() > 380.0 {
        let target = Vec3::new(camera_transform.translation.x, transform.translation.y, camera_transform.translation.z);
        commands.entity(camera).insert(
            camera_transform.ease_to(
                Transform {
                    translation: target
                    , rotation: camera_transform.rotation
                    , scale: camera_transform.scale
                }
                , bevy_easings::EaseFunction::CubicOut
                , bevy_easings::EasingType:: Once {duration: std::time::Duration::from_millis(400)}
            )
        );
    }
}
