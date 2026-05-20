use bevy::prelude::*;

#[derive(Deref)]
pub struct TilemapSprite {
    #[deref]
    pub name: &'static str,
    pub pixel_x: u32,
    pub pixel_y: u32,
}

pub struct TilemapDefinition {
    pub tile_width: u32,
    pub tile_height: u32,
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub sprites: &'static [TilemapSprite],
}

impl TilemapDefinition {
    pub const fn tile_size(&self) -> UVec2 {
        UVec2::new(self.tile_width, self.tile_height)
    }

    pub const fn atlas_size(&self) -> UVec2 {
        UVec2::new(self.atlas_width, self.atlas_height)
    }

    pub fn sprite_index(&self, name: &str) -> Option<usize> {
        self.sprites.iter().position(|sprite| sprite.name == name)
    }
}

pub const DECOR_TILEMAP: TilemapDefinition = TilemapDefinition {
    tile_width: 16,
    tile_height: 16,
    atlas_width: 256,
    atlas_height: 256,
    sprites: &[
        // --- 2 LARGE BUSHES ---
        TilemapSprite {
            name: "bush_lg_g_0_0",
            pixel_x: 1,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "bush_lg_g_0_1",
            pixel_x: 2,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "bush_lg_g_1_0",
            pixel_x: 1,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "bush_lg_g_1_1",
            pixel_x: 2,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "bush_lg_d_0_0",
            pixel_x: 1,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "bush_lg_d_0_1",
            pixel_x: 2,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "bush_lg_d_1_0",
            pixel_x: 1,
            pixel_y: 3,
        },
        TilemapSprite {
            name: "bush_lg_d_1_1",
            pixel_x: 2,
            pixel_y: 3,
        },
        // --- 2 LOGS ---
        TilemapSprite {
            name: "log_g_0_0",
            pixel_x: 4,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "log_g_0_1",
            pixel_x: 5,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "log_g_0_2",
            pixel_x: 6,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "log_g_1_0",
            pixel_x: 4,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "log_g_1_1",
            pixel_x: 5,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "log_g_1_2",
            pixel_x: 6,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "log_d_0_0",
            pixel_x: 4,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "log_d_0_1",
            pixel_x: 5,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "log_d_0_2",
            pixel_x: 6,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "log_d_1_0",
            pixel_x: 4,
            pixel_y: 3,
        },
        TilemapSprite {
            name: "log_d_1_1",
            pixel_x: 5,
            pixel_y: 3,
        },
        TilemapSprite {
            name: "log_d_1_2",
            pixel_x: 6,
            pixel_y: 3,
        },
        // --- 2 SMALL BUSHES ---
        TilemapSprite {
            name: "bush_sm_g_0_0",
            pixel_x: 8,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "bush_sm_g_0_1",
            pixel_x: 8,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "bush_sm_g_1_0",
            pixel_x: 9,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "bush_sm_g_1_1",
            pixel_x: 9,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "bush_sm_d_0_0",
            pixel_x: 8,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "bush_sm_d_0_1",
            pixel_x: 9,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "bush_sm_d_1_0",
            pixel_x: 8,
            pixel_y: 3,
        },
        TilemapSprite {
            name: "bush_sm_d_1_1",
            pixel_x: 9,
            pixel_y: 3,
        },
        // --- 9 MUSHROOMS ---
        TilemapSprite {
            name: "mush_g1_0_0",
            pixel_x: 10,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "mush_g1_0_1",
            pixel_x: 11,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "mush_g1_1_0",
            pixel_x: 10,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "mush_g1_1_1",
            pixel_x: 11,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "mush_g2_0_0",
            pixel_x: 12,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "mush_g2_0_1",
            pixel_x: 13,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "mush_g2_1_0",
            pixel_x: 12,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "mush_g2_1_1",
            pixel_x: 13,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "mush_g3_0_0",
            pixel_x: 14,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "mush_g3_0_1",
            pixel_x: 15,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "mush_g3_1_0",
            pixel_x: 14,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "mush_g3_1_1",
            pixel_x: 15,
            pixel_y: 1,
        },
        TilemapSprite {
            name: "mush_e1_0_0",
            pixel_x: 10,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "mush_e1_0_1",
            pixel_x: 11,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "mush_e1_1_0",
            pixel_x: 10,
            pixel_y: 3,
        },
        TilemapSprite {
            name: "mush_e1_1_1",
            pixel_x: 11,
            pixel_y: 3,
        },
        TilemapSprite {
            name: "mush_e2_0_0",
            pixel_x: 12,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "mush_e2_0_1",
            pixel_x: 13,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "mush_e2_1_0",
            pixel_x: 12,
            pixel_y: 3,
        },
        TilemapSprite {
            name: "mush_e2_1_1",
            pixel_x: 13,
            pixel_y: 3,
        },
        TilemapSprite {
            name: "mush_e3_0_0",
            pixel_x: 14,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "mush_e3_0_1",
            pixel_x: 15,
            pixel_y: 2,
        },
        TilemapSprite {
            name: "mush_e3_1_0",
            pixel_x: 14,
            pixel_y: 3,
        },
        TilemapSprite {
            name: "mush_e3_1_1",
            pixel_x: 15,
            pixel_y: 3,
        },
        TilemapSprite {
            name: "mush_d1_0_0",
            pixel_x: 10,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "mush_d1_0_1",
            pixel_x: 11,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "mush_d1_1_0",
            pixel_x: 10,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "mush_d1_1_1",
            pixel_x: 11,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "mush_d2_0_0",
            pixel_x: 12,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "mush_d2_0_1",
            pixel_x: 13,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "mush_d2_1_0",
            pixel_x: 12,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "mush_d2_1_1",
            pixel_x: 13,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "mush_d3_0_0",
            pixel_x: 14,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "mush_d3_0_1",
            pixel_x: 15,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "mush_d3_1_0",
            pixel_x: 14,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "mush_d3_1_1",
            pixel_x: 15,
            pixel_y: 5,
        },
        // --- 2 WEEDS & 2 REEDS ---
        TilemapSprite {
            name: "weed_g1_0_0",
            pixel_x: 0,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "weed_g1_0_1",
            pixel_x: 1,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "weed_g1_1_0",
            pixel_x: 0,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "weed_g1_1_1",
            pixel_x: 1,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "weed_g2_0_0",
            pixel_x: 2,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "weed_g2_0_1",
            pixel_x: 3,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "weed_g2_1_0",
            pixel_x: 2,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "weed_g2_1_1",
            pixel_x: 3,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "weed_d1_0_0",
            pixel_x: 4,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "weed_d1_0_1",
            pixel_x: 5,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "weed_d1_1_0",
            pixel_x: 4,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "weed_d1_1_1",
            pixel_x: 5,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "weed_d2_0_0",
            pixel_x: 6,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "weed_d2_0_1",
            pixel_x: 7,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "weed_d2_1_0",
            pixel_x: 6,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "weed_d2_1_1",
            pixel_x: 7,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "reed_g1_0_0",
            pixel_x: 4,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "reed_g1_0_1",
            pixel_x: 5,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "reed_g1_1_0",
            pixel_x: 4,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "reed_g1_1_1",
            pixel_x: 5,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "reed_g2_0_0",
            pixel_x: 6,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "reed_g2_0_1",
            pixel_x: 7,
            pixel_y: 4,
        },
        TilemapSprite {
            name: "reed_g2_1_0",
            pixel_x: 6,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "reed_g2_1_1",
            pixel_x: 7,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "reed_d1_0_0",
            pixel_x: 4,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "reed_d1_0_1",
            pixel_x: 5,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "reed_d1_1_0",
            pixel_x: 4,
            pixel_y: 6,
        },
        TilemapSprite {
            name: "reed_d1_1_1",
            pixel_x: 5,
            pixel_y: 6,
        },
        TilemapSprite {
            name: "reed_d2_0_0",
            pixel_x: 6,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "reed_d2_0_1",
            pixel_x: 7,
            pixel_y: 5,
        },
        TilemapSprite {
            name: "reed_d2_1_0",
            pixel_x: 6,
            pixel_y: 6,
        },
        TilemapSprite {
            name: "reed_d2_1_1",
            pixel_x: 7,
            pixel_y: 6,
        },
        // --- PEBBLE ---
        TilemapSprite {
            name: "pebble_g_0_0",
            pixel_x: 8,
            pixel_y: 6,
        },
        TilemapSprite {
            name: "pebble_g_0_1",
            pixel_x: 9,
            pixel_y: 6,
        },
        TilemapSprite {
            name: "pebble_g_1_0",
            pixel_x: 8,
            pixel_y: 7,
        },
        TilemapSprite {
            name: "pebble_g_1_1",
            pixel_x: 9,
            pixel_y: 7,
        },
        // --- 3 ROCKS ---
        TilemapSprite {
            name: "rock_g1_0_0",
            pixel_x: 160,
            pixel_y: 112,
        },
        TilemapSprite {
            name: "rock_g1_0_1",
            pixel_x: 176,
            pixel_y: 112,
        },
        TilemapSprite {
            name: "rock_g1_1_0",
            pixel_x: 160,
            pixel_y: 128,
        },
        TilemapSprite {
            name: "rock_g1_1_1",
            pixel_x: 176,
            pixel_y: 128,
        },
        TilemapSprite {
            name: "rock_g2_0_0",
            pixel_x: 192,
            pixel_y: 112,
        },
        TilemapSprite {
            name: "rock_g2_0_1",
            pixel_x: 208,
            pixel_y: 112,
        },
        TilemapSprite {
            name: "rock_g2_1_0",
            pixel_x: 192,
            pixel_y: 128,
        },
        TilemapSprite {
            name: "rock_g2_1_1",
            pixel_x: 208,
            pixel_y: 128,
        },
        TilemapSprite {
            name: "rock_g3_0_0",
            pixel_x: 224,
            pixel_y: 112,
        },
        TilemapSprite {
            name: "rock_g3_0_1",
            pixel_x: 240,
            pixel_y: 112,
        },
        TilemapSprite {
            name: "rock_g3_1_0",
            pixel_x: 224,
            pixel_y: 128,
        },
        TilemapSprite {
            name: "rock_g3_1_1",
            pixel_x: 240,
            pixel_y: 128,
        },
        // --- 3 BOULDERS ---
        TilemapSprite {
            name: "boulder_g1_0_0",
            pixel_x: 160,
            pixel_y: 144,
        },
        TilemapSprite {
            name: "boulder_g1_0_1",
            pixel_x: 176,
            pixel_y: 144,
        },
        TilemapSprite {
            name: "boulder_g1_1_0",
            pixel_x: 160,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "boulder_g1_1_1",
            pixel_x: 176,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "boulder_g2_0_0",
            pixel_x: 192,
            pixel_y: 144,
        },
        TilemapSprite {
            name: "boulder_g2_0_1",
            pixel_x: 208,
            pixel_y: 144,
        },
        TilemapSprite {
            name: "boulder_g2_1_0",
            pixel_x: 192,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "boulder_g2_1_1",
            pixel_x: 208,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "boulder_g3_0_0",
            pixel_x: 224,
            pixel_y: 144,
        },
        TilemapSprite {
            name: "boulder_g3_0_1",
            pixel_x: 240,
            pixel_y: 144,
        },
        TilemapSprite {
            name: "boulder_g3_1_0",
            pixel_x: 224,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "boulder_g3_1_1",
            pixel_x: 240,
            pixel_y: 160,
        },
        // --- 2 OAK TREES ---
        // Grass Base
        TilemapSprite {
            name: "oak_g_0_0",
            pixel_x: 0,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "oak_g_0_1",
            pixel_x: 16,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "oak_g_0_2",
            pixel_x: 32,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "oak_g_0_3",
            pixel_x: 48,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "oak_g_0_4",
            pixel_x: 64,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "oak_g_1_0",
            pixel_x: 0,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "oak_g_1_1",
            pixel_x: 16,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "oak_g_1_2",
            pixel_x: 32,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "oak_g_1_3",
            pixel_x: 48,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "oak_g_1_4",
            pixel_x: 64,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "oak_g_2_0",
            pixel_x: 0,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "oak_g_2_1",
            pixel_x: 16,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "oak_g_2_2",
            pixel_x: 32,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "oak_g_2_3",
            pixel_x: 48,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "oak_g_2_4",
            pixel_x: 64,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "oak_g_3_0",
            pixel_x: 0,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "oak_g_3_1",
            pixel_x: 16,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "oak_g_3_2",
            pixel_x: 32,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "oak_g_3_3",
            pixel_x: 48,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "oak_g_3_4",
            pixel_x: 64,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "oak_g_4_0",
            pixel_x: 0,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "oak_g_4_1",
            pixel_x: 16,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "oak_g_4_2",
            pixel_x: 32,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "oak_g_4_3",
            pixel_x: 48,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "oak_g_4_4",
            pixel_x: 64,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "oak_g_5_0",
            pixel_x: 0,
            pixel_y: 240,
        },
        TilemapSprite {
            name: "oak_g_5_1",
            pixel_x: 16,
            pixel_y: 240,
        },
        TilemapSprite {
            name: "oak_g_5_2",
            pixel_x: 32,
            pixel_y: 240,
        },
        TilemapSprite {
            name: "oak_g_5_3",
            pixel_x: 48,
            pixel_y: 240,
        },
        TilemapSprite {
            name: "oak_g_5_4",
            pixel_x: 64,
            pixel_y: 240,
        },
        // Dirt Base
        TilemapSprite {
            name: "oak_d_0_0",
            pixel_x: 80,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "oak_d_0_1",
            pixel_x: 96,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "oak_d_0_2",
            pixel_x: 112,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "oak_d_0_3",
            pixel_x: 128,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "oak_d_0_4",
            pixel_x: 144,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "oak_d_1_0",
            pixel_x: 80,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "oak_d_1_1",
            pixel_x: 96,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "oak_d_1_2",
            pixel_x: 112,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "oak_d_1_3",
            pixel_x: 128,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "oak_d_1_4",
            pixel_x: 144,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "oak_d_2_0",
            pixel_x: 80,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "oak_d_2_1",
            pixel_x: 96,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "oak_d_2_2",
            pixel_x: 112,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "oak_d_2_3",
            pixel_x: 128,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "oak_d_2_4",
            pixel_x: 144,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "oak_d_3_0",
            pixel_x: 80,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "oak_d_3_1",
            pixel_x: 96,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "oak_d_3_2",
            pixel_x: 112,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "oak_d_3_3",
            pixel_x: 128,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "oak_d_3_4",
            pixel_x: 144,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "oak_d_4_0",
            pixel_x: 80,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "oak_d_4_1",
            pixel_x: 96,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "oak_d_4_2",
            pixel_x: 112,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "oak_d_4_3",
            pixel_x: 128,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "oak_d_4_4",
            pixel_x: 144,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "oak_d_5_0",
            pixel_x: 80,
            pixel_y: 240,
        },
        TilemapSprite {
            name: "oak_d_5_1",
            pixel_x: 96,
            pixel_y: 240,
        },
        TilemapSprite {
            name: "oak_d_5_2",
            pixel_x: 112,
            pixel_y: 240,
        },
        TilemapSprite {
            name: "oak_d_5_3",
            pixel_x: 128,
            pixel_y: 240,
        },
        TilemapSprite {
            name: "oak_d_5_4",
            pixel_x: 144,
            pixel_y: 240,
        },
        // --- 2 PINE TREES ---
        // Grass Base
        TilemapSprite {
            name: "pine_g_0_0",
            pixel_x: 176,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "pine_g_0_1",
            pixel_x: 192,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "pine_g_0_2",
            pixel_x: 208,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "pine_g_1_0",
            pixel_x: 176,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "pine_g_1_1",
            pixel_x: 192,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "pine_g_1_2",
            pixel_x: 208,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "pine_g_2_0",
            pixel_x: 176,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "pine_g_2_1",
            pixel_x: 192,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "pine_g_2_2",
            pixel_x: 208,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "pine_g_3_0",
            pixel_x: 176,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "pine_g_3_1",
            pixel_x: 192,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "pine_g_3_2",
            pixel_x: 208,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "pine_g_4_0",
            pixel_x: 176,
            pixel_y: 240,
        },
        TilemapSprite {
            name: "pine_g_4_1",
            pixel_x: 192,
            pixel_y: 240,
        },
        TilemapSprite {
            name: "pine_g_4_2",
            pixel_x: 208,
            pixel_y: 240,
        },
        // Bare Base
        TilemapSprite {
            name: "pine_b_0_0",
            pixel_x: 208,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "pine_b_0_1",
            pixel_x: 224,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "pine_b_0_2",
            pixel_x: 240,
            pixel_y: 176,
        },
        TilemapSprite {
            name: "pine_b_1_0",
            pixel_x: 208,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "pine_b_1_1",
            pixel_x: 224,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "pine_b_1_2",
            pixel_x: 240,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "pine_b_2_0",
            pixel_x: 208,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "pine_b_2_1",
            pixel_x: 224,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "pine_b_2_2",
            pixel_x: 240,
            pixel_y: 208,
        },
        TilemapSprite {
            name: "pine_b_3_0",
            pixel_x: 208,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "pine_b_3_1",
            pixel_x: 224,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "pine_b_3_2",
            pixel_x: 240,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "pine_b_4_0",
            pixel_x: 208,
            pixel_y: 240,
        },
        TilemapSprite {
            name: "pine_b_4_1",
            pixel_x: 224,
            pixel_y: 240,
        },
        TilemapSprite {
            name: "pine_b_4_2",
            pixel_x: 240,
            pixel_y: 240,
        },
    ],
};
