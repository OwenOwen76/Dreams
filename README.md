# Dreams

This is a 2D Singleplayer Top-Down Procedural Survival RPG, where the player explores an open, procedurally generated world, engaging in real-time combat, gathering resources, completing quests, and interacting with NPCs, and uncover the secrets of this world. Each in-game day lasts approximately 30 minutes and ends at Midnight, where the entire world resets with a new procedural seed.

Overview.md has more info on this and my current progress, don't ask why I've organized it like this because I have no idea either. Note that Overview.md contains basically my entire outline for my game so reading everything there will significantly impact your gameplay experience.

The NPC system for this game is [here](https://github.com/OwenOwen76/NPC_Dialog_System) right now because using Bevy adds another layer of complexity that I do not need.

This game is still in the early developmental phrases and therefore please do not judge the game based on what you see now. However feedbacks are welcome, make a pull requewst, report a issue, or join my [Discord server](https://discord.gg/F59n6Mj7pj)

Have a nice day~

## Running on the Web

Follow the instructions in wasm.md to run this locally on the web, however it is not recommended because there is significant downgrade in quality and the camera is especially laggy. Most of the errors is mostly likely due to compiling differences than was created when the files was converted into Python. This game is also available (for now, it could be shut down anytime without warning) on https://owenowen76.github.io/

**there a lot of issues with this right now so don't expect it to work, ignore everything written up there, thx for understanding**

## TODO/Issues

### MAP 
#### Map bitmasking sometimes fails
(grass example)

tr

mc tr

mc tc tr

mc mc mc tr

the tc above should have been a mc

#### Map water-grass connection
right now when water is beside grass both generates the border, grass should instead generate grass_mc and connect with water_grass.

#### Map doesn't despwn entities
map doesn't despawn entities that out of render distance

#### Tree sprite getting cut off
the top of the big trees doesn't generate and is cut off
the top and right of small tree doesn't generate and is cut off

#### Boulders and Large Bushes are spawning together as a bundle
the bottom two boulder sprites and the top two large bush sprite are being spawned togther as a bundle for some reason

#### Tree placement errors
tree in water, tree too close (on top of each other), in very small patches

#### Decors placement via blue noise
right now the decors of the map is spawning in rigid grids, needs to fixed with more detailed placement via blue noise

### Player 
#### Player jump is animation only
when the player jump w/ space bar it's a animation that plays but nothing really happens

#### Player z-sorting logic is not working
the z-sorting for the player is not work, most likely the map decors are loading on the wrong z-layer

### Other 
#### NPC & Pathfinding
both folders are place holders, empty at the moment

## Credits

Character codes are based off [The Impatient Programmer's Guide to Bevy and Rust](https://aibodh.com/)

Perlin Niose implementation is based off the original implementation from the [niose crate](https://crates.io/crates/noise)

Character Design: [Universal LPC Spritesheet Generator](https://liberatedpixelcup.github.io/Universal-LPC-Spritesheet-Character-Generator/#sex=male&body=Body_Color_light&head=Human_Male_light&expression=Neutral_light)

Map Tileset: [Free Topdown Fantasy - Forest - Pixelart Tileset](https://aamatniekss.itch.io/topdown-fantasy-forest)
