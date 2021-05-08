use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people.system())
        .add_system(greet_people.system())
        .run();
}

#[derive(Debug)]
struct Human {
    age: u8,
}

struct BaseStats {
    intelligence: u8,
    strength: u8,
    speed: u8,
    aura: u8, // Used to boost your magic skills, as most magic includes this stat or derived stats
}

#[derive(Debug)]
struct Name(String);
//System

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(BaseStats {
            intelligence: 8,
            strength: 8,
            speed: 8,
            aura: 8,
        })
        .insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Name("Elaina penis".to_string()));
    commands
        .spawn()
        .insert(Human { age: 3 })
        .insert(Name("Renzo Hume".to_string()));
    commands
        .spawn()
        .insert(Human { age: 2 })
        .insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(query: Query<(&Name, &BaseStats)>) {
    for name in query.iter() {
        println!("Hello {:?} of age {}!", name.0 .0, name.1.age);
    }
}

//entity
