use bevy::prelude::*;
use std::thread;

pub mod config;
mod discord;
mod magic;
pub mod timesys;
mod uisys;
mod world;

fn main() {
    let config = config::load_config();
    if config.discordrpc.unwrap_or(true) == true {
        thread::spawn(|| {
            discord::discord_status();
        });
    }
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Arche Universe".to_string(),
            vsync: config.vsync.unwrap_or(false),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(timesys::DateTimeSystem)
        .add_plugin(uisys::UISystem)
        .add_plugin(StartupSystem)
        .run();
}

#[derive(Debug)]
struct Name(String);
//System
fn add_people(mut commands: Commands) {
    commands.spawn().insert(Name("Elaina Proctor".to_string()));
    //.insert(Aether(6));

    commands.spawn().insert(Name("Elaina penis".to_string()));
}

pub struct StartupSystem;
impl Plugin for StartupSystem {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_people.system())
            //.add_startup_system(world::setup.system())
            .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)));
    }
}

pub struct Player {}
