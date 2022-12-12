# Crab's pipes

Game where you play as a crab in pipes. You can go left.. or right. Thats all.

TODO:
- [X] Draw a crab
- [X] Walking scene
- [X] Camera movement
- [X] win/lin/wasm builds
- [X] Macroquad storage/scene look
- [X] Add wall climbing
- [X] Crab animation
- [X] Crab idle animation
- [X] Menu
- [X] Item
- [X] SFX
- [X] Ambient music
- [X] Tile graphics
- [ ] Background
- [ ] Button/valve system
- [ ] Levels

## Creating levels

You can add levels to the game using (Tiled)[https://www.mapeditor.org/]. Create level by copying from existing levels (`assets/level{,_2}.json`) and edit it as you wish.
There should be `main` layer with pipes and `diamond` layer with diamond. After creating you should export it as json to `assets/<level_name>.json` and insert in `src/assets.rs`:
```rust

const TILEMAPS: [&str; /*new number of levels*/] = [
...

    include_str!("../assets/<level_name>.json"),
];
```
