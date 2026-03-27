### --- TODO ---

## --- MAP ---
# Map bitmasking sometimes fails
(grass example)
tr
mc tr
mc tc tr
mc mc mc tr
the tc above should have been a mc

# Map water-grass connection
right now when water is beside grass both generates the border, grass should instead generate grass_mc and connect with water_grass.

# Map doesn't despwn entities
map doesn't despawn entities that out of render distance

# Tree sprite getting cut off
the top of the big trees doesn't generate and is cut off
the top and right of small tree doesn't generate and is cut off

# Tree placement errors
tree in water, tree too close (on top of each other), in very small patches

## --- Player ---
# Player jump is animation only
when the player jump w/ space bar it's a animation that plays but nothing really happens

# Player z-sorting logic is not working
the z-sorting for the player is not work, most likely the map decors are loading on the wrong z-layer

## --- Other ---
# npc & pathfinding
both folders are place holders, both empty
