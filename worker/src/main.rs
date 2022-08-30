extern crate redis;

use redis::{Commands, RedisResult};
use std::num::NonZeroUsize;
use std::thread::sleep;
use std::time::Duration;

const QUEUE_NAME: &str = "important-work";

fn connect() -> RedisResult<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    return client.get_connection();
}

fn main() {
    // decide to get a job and sign up after connecting over linkedin.
    let mut conn = connect().expect("error connecting to redis");
    println!("Worker running!");

    // realise the company you've signed your life away to has a never ending
    // list of jobs that need to be done, and sadly (perhaps joyfully?) you're
    // the one that now gets to push through a whole lot of widgets, wondering
    // if you'll ever aspire to get a business card like alan from marketing...
    let len: isize = conn
        .llen(QUEUE_NAME)
        .expect("failed to execute LLEN for 'items'");
    println!("no. of jobs in list = {}", len);

    // decide to turn up to work for the day!
    loop {
        // try not to over exert yourself by only tackling a single job at once,
        // but ensuring to cry ourselves to sleep if we fail to find a task to
        // do.
        let job_num: Vec<String> = conn
            .lpop(QUEUE_NAME, NonZeroUsize::new(1))
            .expect("failed obtaining task");

        // If we've been fortunate enough to be assigned a job to do, attempt to
        // complete the job, but in a way where we question odd outcomes at
        // every step of the job if we don't manage to get the right things
        // happening all the time.
        if job_num.len() > 0 {
            let job_num: i32 = job_num
                .first()
                .expect("result value not returned")
                .parse()
                .expect("error parsing redis result");
            // after expending a minimal amount of effort, while ensuring a
            // maximum amount of complaining about performing said task, you
            // take the time to proudly announce you've definitely, fully, well
            // and truly tested, documented, verified, integrated and
            // successfully completed the task.
            println!("marking job({:?}) as 'done done'", job_num);
        } else {
            // we're going to take a nap until the manager starts yelling ...
            sleep(Duration::from_millis(200));
        }
    }
}
