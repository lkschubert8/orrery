use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use bevy_diagnostic::*;
mod components;
mod entities;
mod systems;

#[derive(Debug)]
struct Location {
    x: u64,
    y: u64,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let transparent_handle = asset_server
        .load_sync(&mut textures, "assets/colored_transparent_packed.png")
        .unwrap();
    let transparent_texture = textures.get(&transparent_handle).unwrap();
    let transparent_texture_atlas = TextureAtlas::from_grid(transparent_handle, transparent_texture.size, 48, 22);
    let transparent_texture_atlas_handle = texture_atlases.add(transparent_texture_atlas);
 
    commands
        // cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: transparent_texture_atlas_handle,
            translation: Translation(Vec3::new(0.0, 0.0, 100.0)),
            scale: Scale(4.0),
            sprite: TextureAtlasSprite::new(362),
            ..Default::default()
        })
        .with(entities::Player);
}

fn main() {  
    App::build()
        .add_default_plugins()
        .add_plugin(bevy_diagnostic::FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(systems::asset_loading::map::load_map.system())
        .add_resource(components::WalkTimer(Timer::from_seconds(0.33)))
        .add_system(systems::player_movement::handle_input.system())
        .run();
}
