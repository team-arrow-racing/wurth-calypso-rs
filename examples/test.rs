#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use std::env;
use std::time::Duration;

use atat::{
    asynch::AtatClient, AtatIngress, Buffers, Config, DefaultDigester, Ingress,
};
use embassy_time; // this stops a linker error cause embassy is dumb
use embedded_io_adapters::tokio_1::FromTokio;
use tokio::io::split;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::ReadHalf;
use tokio_serial::SerialPortBuilderExt;
use tokio_serial::SerialStream;
use wurth_calypso::command;
use wurth_calypso::command::Urc;

// Standard UART parameters
// Baud: 921600
// Data bits: 8
// Stop bits: 1
// Parity: even
// Flow control: none

const INGRESS_BUF_SIZE: usize = 1024;
const URC_CAPACITY: usize = 128;
const URC_SUBSCRIBERS: usize = 3;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let port_name = args.get(1).expect("Please provide a serial port");

    println!("Serial port: {:?}", port_name);

    // we open this port twice so we have a reader and writer instance.
    // yes, `tokio::io::split` exists, but it doesn't work with `tokio_serial`.
    let builder = tokio_serial::new(port_name, 921600)
        .data_bits(tokio_serial::DataBits::Eight)
        .stop_bits(tokio_serial::StopBits::One)
        .parity(tokio_serial::Parity::Even);

    let mut reader = builder
        .clone()
        .open_native_async()
        .expect("Failed to open serial port");
    reader.set_exclusive(false).unwrap();

    let mut writer = builder
        .clone()
        .open_native_async()
        .expect("Failed to open serial port");
    writer.set_exclusive(false).unwrap();

    static BUFFERS: Buffers<
        Urc,
        INGRESS_BUF_SIZE,
        URC_CAPACITY,
        URC_SUBSCRIBERS,
    > = Buffers::<Urc, INGRESS_BUF_SIZE, URC_CAPACITY, URC_SUBSCRIBERS>::new();

    let (ingress, mut client) = BUFFERS.split(
        FromTokio::new(writer),
        DefaultDigester::<Urc>::default(),
        Config::default(),
    );

    tokio::spawn(ingress_task(ingress, FromTokio::new(reader)));

    loop {
        let response = client.send(&command::device::Test {}).await;
        println!("Response: {:?}", response);

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn ingress_task<'a>(
    mut ingress: Ingress<
        'a,
        DefaultDigester<Urc>,
        Urc,
        INGRESS_BUF_SIZE,
        URC_CAPACITY,
        URC_SUBSCRIBERS,
    >,
    mut reader: FromTokio<SerialStream>,
) -> ! {
    ingress.read_from(&mut reader).await
}
