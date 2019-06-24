extern crate redis_cluster_rs;

use redis_cluster_rs::{Client, Commands};
use std::time::Instant;
// use std::time::Duration;
use std::thread;

fn main() {
    let nodes = vec![
        "redis://127.0.0.1:7000",
        "redis://127.0.0.1:7001",
        "redis://127.0.0.1:7002",
    ];

    //let reqs = 1;
    let reqs = 1; //00000;

    println!("Connecting to Redis...");
    let client: redis_cluster_rs::Client = Client::open(nodes).unwrap();
    let con = client.get_connection().unwrap();

    assert!(con.check_connection());

    println!("Perform {} iterations of 'set'", reqs);
    let start = Instant::now();

    let handle = thread::spawn(move || {
        for n in 1..=reqs {
            let _: () = con.set(format!("{}", n), "test_data").unwrap();
        }
    });
    handle.join().unwrap();

    // let send_requests = |con: &redis_cluster_rs::Connection, num| {
    //     for n in 1..=num {
    //         let _: () = con.set(format!("{}", n), "test_data").unwrap();
    //     }
    // };

    // send_requests(&con, reqs);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    // // println!("Get data");
    // let res: String = con.get("test").unwrap();
    // assert_eq!(res, "test_data");

    println!("Done");
}
