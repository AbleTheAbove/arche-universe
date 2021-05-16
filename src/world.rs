use bevy::{
    asset::{AssetServer, Assets},
    ecs::system::{Commands, Res, ResMut},
    math::Vec3,
    prelude::IntoSystem,
    render::entity::OrthographicCameraBundle,
    sprite::{entity::SpriteBundle, ColorMaterial},
    transform::components::Transform,
};

pub const CHUNKSIZE: usize = 128;
pub type Chunk = [[Tile; CHUNKSIZE]; CHUNKSIZE];

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Air,
    Stone,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let worldy = Box::new([[Tile::Air; CHUNKSIZE]; CHUNKSIZE]);

    let texture_handle = asset_server.load("uwu.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let xyz = materials.add(texture_handle.into());
    let mut x_cord = -20.0;
    let mut y_cord = 1.0;
    for y in worldy.iter() {
        for x in y {
            let position = Vec3::new(x_cord, y_cord, y_cord);
            let translation = position * 32.0;

            commands.spawn_bundle(SpriteBundle {
                material: xyz.clone(),
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                ..Default::default()
            });
            x_cord += 1.0;
        }
        y_cord += 1.0;
    }
}
