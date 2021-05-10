use bevy::{
    app::{AppBuilder, Plugin},
    asset::AssetServer,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    ecs::{
        query::With,
        system::{Commands, Query, Res},
    },
    math::Rect,
    prelude::{HorizontalAlign, IntoSystem},
    render::color::Color,
    text::{Text, TextAlignment, TextSection, TextStyle},
    ui::{
        entity::{TextBundle, UiCameraBundle},
        AlignSelf, PositionType, Style, Val,
    },
};

use crate::timesys;
pub struct UISystem;
impl Plugin for UISystem {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(ui_start.system())
            .add_system(text_update_system.system())
            .add_system(text_update_system2.system());
    }
}

struct FpsText;
pub fn ui_start(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    TextSection {
                        value: "Day: ".to_string(),
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
fn text_update_system2(datetime: Res<timesys::DateTime>, mut query: Query<&mut Text, With<Hax>>) {
    for mut text in query.iter_mut() {
        // Update the value of the second section
        text.sections[1].value = format!("{}\n", datetime.tick);
        text.sections[3].value = format!("{}\n", datetime.day);
    }
}
