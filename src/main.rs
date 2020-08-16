use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
mod player_movement;
mod map;
enum Collider {
    Solid,
    Scorable,
}
struct Actor;
struct CameraTracker;
struct Name(String);
struct GreetTimer(Timer);

struct Count(usize);

#[derive(Debug)]
struct Location {
    x: u64,
    y: u64,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
        .with(Collider::Solid)
        .with(player_movement::Player)
        .with(player_movement::MovementCommandStack {
            directions: Vec::new(),
        });

   
}

fn main() {
    
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_startup_system(map::load_map.system())
        .add_resource(player_movement::WalkTimer(Timer::from_seconds(0.16)))
        .add_system(player_movement::player_handle_input.system())
        .add_system(player_movement::player_handle_movement.system())
        //.add_system(print_player_location.system())
        .run();
}
