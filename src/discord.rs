extern crate discord_rpc_client;

use discord_rpc_client::{
    models::{rich_presence::ActivityParty, Event},
    Client,
};
use std::{thread, time};

pub fn status() {
    // Create the client
    let mut drpc = Client::new(841197292660588545);

    // Start up the client connection, so that we can actually send and receive stuff

    drpc.start();
    drpc.subscribe(Event::ActivityJoin, |j| {
        j.secret("MTI4NzM0OjFpMmhuZToxMjMxMjM= ")
    })
    .expect("Failed to subscribe to event");

    drpc.subscribe(Event::ActivityJoinRequest, |s| {
        println!("{:?}", s);
        s
    })
    .expect("Failed to subscribe to event");

    // Set the activity
    drpc.set_activity(|a| {
        a.state("Arche Universe")
            .assets(|ass| {
                ass.large_image("archeuniverselogo")
                    .large_text("wat.")
                    .small_image("archeuniverselogo")
                    .small_text("Highest Leveled Class Icon")
            })
            .secrets(|a| a.join("MTI4NzM0OjFpMmhuZToxMjMxMjM= "))
            .timestamps(|a| {
                a.start(1620635125)
                //.end(1620653237) // Use for a limited time round (Like uh a duel)
            })
            .details("You thought")
            .party(|a| a.size((1, 5)))
    })
    .expect("Failed to set activity");

    // Wait 10 seconds before exiting
    thread::sleep(time::Duration::from_secs(10));
    loop {}
}
