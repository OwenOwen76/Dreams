use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TileType {
    //Walkable
    #[default]
    Empty,
    Dirt,
    Grass,
    DarkGrass,
    Shore,
    SmallDecors,
    //Non-Walkable
    Water,
    Tree,
    LargeDecors,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        !matches!(
            self,
            TileType::Water | TileType::Tree | TileType::LargeDecors
        )
    }

    pub fn collision_adjustment(&self) -> f32 {
        match self {
            TileType::Tree | TileType::LargeDecors => -0.2,
            _ => 0.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct TileMarker {
    pub tile_type: TileType,
}

impl TileMarker {
    pub fn new(tile_type: TileType) -> Self {
        Self { tile_type }
    }
}
