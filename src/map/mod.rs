use bevy::prelude::*;

extern crate tiled;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tiled::parse;

struct TerrainBlock;
pub enum Collider {
    Level1,
    Level2,
    Blocked
}
pub fn load() -> tiled::Map {
    let file = File::open(&Path::new("assets/test-room.tmx")).unwrap();
    println!("Opened file");
    let reader = BufReader::new(file);
    let map = parse(reader).unwrap();
    println!("{:?}", map);
    map
}



pub fn load_map(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let map = load();
    let background_handle = asset_server
        .load_sync(&mut textures, "assets/colored_packed.png")
        .unwrap();
    let texture = textures.get(&background_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(background_handle, texture.size, 48, 22);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let tile_size = 16.0 * 4.0;
    for layer in &map.layers {
        let is_terrain = match layer
            .properties
            .get("terrain")
            .unwrap_or(&tiled::PropertyValue::BoolValue(false))
        {
            tiled::PropertyValue::BoolValue(val) => val.clone(),
            _ => false,
        };

        let mut y = map.height as f32 * tile_size / 2.0;
        for row in &layer.tiles {
            let mut x = -1.0 * map.width as f32 * tile_size / 2.0;
            for tile in row {
                if is_terrain {
                    let terrain_type = match tile.gid {
                        1 => Collider::Level1,
                        2 => Collider::Level2,
                        3 => Collider::Blocked,
                        _ => Collider::Blocked
                    };
                    commands.spawn((TerrainBlock, terrain_type));
                } else {
                    commands.spawn(SpriteSheetComponents {
                        texture_atlas: texture_atlas_handle,
                        translation: Translation(Vec3::new(x, y, 0.0)), 
                        scale: Scale(4.0),
                        sprite: TextureAtlasSprite::new(tile.gid - 1),
                        ..Default::default()
                    });
                }
                x += tile_size;
            }
            y -= tile_size;
        }
    }
}
