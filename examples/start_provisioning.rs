#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

mod common;

use embassy_time; // this stops a linker error cause embassy is dumb
use std::env;
use std::time::Duration;
use wurth_calypso::Calypso;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let port = args.get(1).expect("Please provide a serial port");

    let client = common::atat_client(port).await;
    let mut calypso = Calypso::new(client);

    let response = calypso.provisioning_start().await;
    println!("Response: {:?}", response);
}
