use crate::components::*;
use crate::entities::*;
use bevy::prelude::*;
extern crate tiled;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tiled::parse;

pub fn load() -> tiled::Map {
    let file = File::open(&Path::new("assets/maps/test-room.tmx")).unwrap();
    println!("Opened file");
    let reader = BufReader::new(file);
    let map = parse(reader).unwrap();
    // println!("{:?}", map);
    map
}

// TODO make this whole thing better
pub fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    //mut maps: ResMut<Assets<File>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let background_handle = asset_server
        .load_sync(&mut textures, "assets/maps/colored_packed.png")
        .unwrap();

    // let map_handle : Handle<File> = asset_server
    //     .load_sync(&mut maps, "assets/test-room.tmx")
    //     .unwrap();
    // let map = parse(maps.get(&map_handle).unwrap()).unwrap();
    let map = load();
    asset_server.watch_for_changes().unwrap();
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
                        1057 => Collider::Level1,
                        1058 => Collider::Level2,
                        1059 => Collider::Blocked,
                        _ => Collider::Blocked,
                    };
                    commands.spawn((
                        TerrainBlock,
                        terrain_type,
                        Translation(Vec3::new(x, y, 0.0)),
                    ));
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
