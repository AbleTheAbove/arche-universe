use bevy::prelude::*;

mod config;
mod timesys;

fn main() {
    let config = config::load_config();
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Able Sandbox".to_string(),
            vsync: config.vsync.unwrap_or(false),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(TickTimer(Timer::from_seconds(2.0, true)))
        .insert_resource(timesys::DateTime { tick: 0 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people.system())
        .add_plugin(DateTimeSystem)
        .run();
}

#[derive(Debug)]
struct Aether(u32);

#[derive(Debug)]
struct Name(String);
//System
fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Name("Elaina Proctor".to_string()))
        .insert(Aether(6));

    commands.spawn().insert(Name("Elaina penis".to_string()));
}
struct TickTimer(Timer);

fn read_mana(
    time: Res<Time>,
    mut timer: ResMut<TickTimer>,
    mut datetime: ResMut<timesys::DateTime>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("{}", datetime.tick);
        datetime.tick += 1;
    }
}

pub struct DateTimeSystem;

impl Plugin for DateTimeSystem {
    fn build(&self, app: &mut AppBuilder) {
        // add things to your app here
        app.add_system(read_mana.system());
    }
}
