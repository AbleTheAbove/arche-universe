/*
One irl hour should be equivelent to a day in game
*/

use bevy::{
    app::{AppBuilder, Plugin},
    core::{Time, Timer},
    ecs::system::{Res, ResMut},
    prelude::IntoSystem,
};

pub struct DateTime {
    pub day: u32,
    pub tick: u16,
}

pub struct TickTimer(pub Timer);

fn tick(time: Res<Time>, mut timer: ResMut<TickTimer>, mut datetime: ResMut<DateTime>) {
    if timer.0.tick(time.delta()).just_finished() {
        //                  65535
        if datetime.tick >= 18000 {
            datetime.tick = 0;
            datetime.day += 1;
            println!("{}", datetime.day);
        }
        datetime.tick += 1;
    }
}

pub struct DateTimeSystem;
impl Plugin for DateTimeSystem {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(tick.system())
            .insert_resource(TickTimer(Timer::from_seconds(0.05, true)))
            .insert_resource(DateTime { day: 0, tick: 0 });
    }
}
