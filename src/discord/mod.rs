extern crate discord_rpc_client;

use discord_rpc_client::{
    models::{rich_presence::ActivityParty, Event},
    Client,
};
use std::{thread, time};

pub fn discord_status() {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Create the client

    let mut drpc = Client::new(841197292660588545);

    // Start up the client connection, so that we can actually send and receive stuff

    drpc.start();
    /*
        drpc.subscribe(Event::ActivityJoin, |j| {
            j.secret("MTI4NzM0OjFpMmhuZToxMjMxMjM= ")
        })
        .expect("Failed to subscribe to event");

        drpc.subscribe(Event::ActivityJoinRequest, |s| {
            println!("{:?}", s);
            s
        })
        .expect("Failed to subscribe to event");

        drpc.unsubscribe(Event::ActivityJoinRequest, |j| {
            println!("{:?}", j);
            j
        })
        .expect("Failed to unsubscribe from event");

    */
    // Set the activity
    drpc.set_activity(|a| {
        a.state(match_status(GameStatus::Menu))
            .assets(|ass| {
                ass.large_image("archeuniverselogo")
                //.large_text("wat.")
                //.small_image("archeuniverselogo")
                //.small_text("Highest Leveled Class Icon")
            })
            //.secrets(|a| a.join("MTI4NzM0OjFpMmhuZToxMjMxMjM= "))
            .timestamps(|a| {
                let start = SystemTime::now();
                let since_the_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                let in_ms = since_the_epoch.as_secs() * 1000
                    + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

                a.start(in_ms)
                //.end(1620653237) // Use for a limited time round (Like uh a duel)
            })
        //.details("You thought")
        //.party(|a| a.size((1, 16)))
    })
    .expect("Failed to set activity");

    // Wait 10 seconds before exiting
    thread::sleep(time::Duration::from_secs(10));
    loop {}
}

pub enum GameStatus {
    Menu,
    InGame,
    Duel,
}

fn match_status(status: GameStatus) -> String {
    match status {
        GameStatus::InGame => "Exploring...".to_string(),
        GameStatus::Menu => "In the menu.".to_string(),
        GameStatus::Duel => "Dueling".to_string(),
    }
}
