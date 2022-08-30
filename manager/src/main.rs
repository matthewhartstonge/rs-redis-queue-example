extern crate redis;

use redis::{Commands, RedisResult};
use std::thread::sleep;
use std::time::Duration;

const QUEUE_NAME: &str = "important-work";

fn connect() -> RedisResult<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    return client.get_connection();
}

fn main() {
    // decide it's about time we start marketing ourselves on linkedin, cause
    // we've heard there is some great talent on there.
    let mut conn = connect().expect("error connecting to redis");

    println!("Manager running!");

    // Set up a really important meeting (right now) that everybody has to
    // listen to (...forever...)
    let batch_size = 10;
    let mut batch = 0;
    loop {
        // calculate a manageable amount of work for the minions...
        let batch_start = batch * batch_size;
        let batch_end = batch * batch_size + batch_size;

        // RE: RE: fwd: RE: RE: ALL THE WORK THAT NEEDS TO BE DONE RIGHT NOW
        for i in batch_start..batch_end {
            // add a "it'll only take a minute" amount of points to the sprint
            // board.
            let _: () = conn.rpush(QUEUE_NAME, i).unwrap();
            println!("sending job: {}", i);
        }

        // set ourselves up for some astute "AS PER MY LAST EMAIL" managerial
        // action...
        batch += 1;

        // ... but have a break first, cause we've earned it.
        sleep(Duration::from_millis(100));
    }
}
