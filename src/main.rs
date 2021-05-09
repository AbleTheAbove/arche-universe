use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
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
        .insert_resource(TickTimer(Timer::from_seconds(1.0, true)))
        .insert_resource(timesys::DateTime { tick: 0 })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(add_people.system())
        .add_startup_system(startup.system())
        .add_system(text_update_system.system())
        .add_system(text_update_system2.system())
        .add_plugin(DateTimeSystem)
        .run();
}
struct FpsText;
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    // Rich text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Tick: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("Noto_Serif/NotoSerif-Regular.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("Noto_Serif/NotoSerif-Regular.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Hax);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("Noto_Serif/NotoSerif-Regular.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("Noto_Serif/NotoSerif-Regular.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}
struct Hax;
fn text_update_system2(
    diagnostics: Res<Diagnostics>,
    datetime: Res<timesys::DateTime>,
    mut query: Query<&mut Text, With<Hax>>,
) {
    for mut text in query.iter_mut() {
        // Update the value of the second section
        text.sections[1].value = format!("{}", datetime.tick);
    }
}

#[derive(Debug)]
struct Name(String);
//System
fn add_people(mut commands: Commands) {
    commands.spawn().insert(Name("Elaina Proctor".to_string()));
    //.insert(Aether(6));

    commands.spawn().insert(Name("Elaina penis".to_string()));
}
struct TickTimer(Timer);

fn tick(time: Res<Time>, mut timer: ResMut<TickTimer>, mut datetime: ResMut<timesys::DateTime>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("{}", datetime.tick);
        datetime.tick += 1;
    }
}

pub struct DateTimeSystem;
impl Plugin for DateTimeSystem {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(tick.system());
    }
}
