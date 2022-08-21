# A Balancing Algorithm: 

An algorithm to balance clusters/groups. Definitely not optimial, but usable in many cases such as power balancing in grid based games or territory balancing in real life businesses (say for your sales team or marketing).

The set up for the algorithm is the following:

1. A notion of a map.
2. A notion of points on the map, and each point belongs to a contiguous group.
3. A notion of distance or neighbors. We can have neighbors without needing distance. But if we have distance, we can use that to define neighbors of the point. (An example of neighbors without distance: we can have zip codes that border each other, but it doesn't make much sense to say zipcode1 is 'distance' far away from zipcode2.)

In the demo, a map is a 2D grid map, and the distance is Manhattan distance. In reality, any distance can be used. The algorithm then proceeds as follows:

1. Find the group with minimun value (any aggregated value that you want to balance). Call the group G1.
2. Find in G1 a point that has non-empty intersection with other groups, call the point p1. (Within radius r of the point p1, there are other points that belong to other groups.)
3. Test the points, and find points that can improve balance, aka decrease the balance_measure. See definition of balance_measure below.
    
    a. If we can find points that can improve balance, then we have 3 execution plans. Any plan will improve balance. Then repeat from 1 until we reach our optimization threshold.

    b. If we cannot find such points, start from Step 2 again until we exhaust all choices. If we still cannot find points that improve balance. Terminate.

There are of course special cases and remedy to those special cases, but I don't feel like going into those and would like to keep the description simple.

### Execution Plans:

1. Eager: once we find a point that can improve balance, we take in the point to the cluster/group that p1 belongs to and repeat the process.
2. Greedy: we find all points that can improve the balance, and we take in the point that improves the balance the most.
3. Distance: we find all points that can improve the balance, and we take in the point that is the most favorable in terms of distance. It can be distance to p1 or distance to the 'center' of the group. Using this will produce the most contiguous outcome.

In the game demo, eager execution is used.

### Balance Measure:

Let say we have 3 clusters, call the aggregated value for the clusters 

$C_1, C_2, C_3$

Then we define our balance measure to be 

$\frac{|C_1 - C_2| + |C_1 - C_3| + |C_2 - C_3|}{3 \choose 2}$

Notice that in perfectly balanced clusters, the numerator will be 0. The denominator is just a normalization factor which we can ignore. This metric easily generalizes to any number of clusters.



# Credits:
I learned Bevy for this demo. A lot of thanks to Logic Project's helpful videos.

https://www.youtube.com/watch?v=WN0XK8wddac

Sprite sheet is also taken from his project.

Video of the demo:


