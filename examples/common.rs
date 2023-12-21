use atat::asynch::Client;
use atat::{AtatIngress, Buffers, Config, DefaultDigester, Ingress};
use embedded_io_adapters::tokio_1::FromTokio;
use tokio::io::split;
use tokio_serial::SerialPortBuilderExt;
use tokio_serial::SerialStream;
use wurth_calypso::command;

const INGRESS_BUF_SIZE: usize = 1024;
const URC_CAPACITY: usize = 128;
const URC_SUBSCRIBERS: usize = 3;

pub async fn atat_client(
    port: &String,
) -> Client<'static, FromTokio<SerialStream>, INGRESS_BUF_SIZE> {
    // we open this port twice so we have a reader and writer instance.
    // yes, `tokio::io::split` exists, but it doesn't work with `tokio_serial`.
    let builder = tokio_serial::new(port, 921600)
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
        command::Urc,
        INGRESS_BUF_SIZE,
        URC_CAPACITY,
        URC_SUBSCRIBERS,
    > = Buffers::<command::Urc, INGRESS_BUF_SIZE, URC_CAPACITY, URC_SUBSCRIBERS>::new();

    let (mut ingress, mut client) = BUFFERS.split(
        FromTokio::new(writer),
        DefaultDigester::<command::Urc>::default(),
        Config::default(),
    );

    tokio::spawn(async move {
        ingress.read_from(&mut FromTokio::new(reader)).await;
    });

    client
}
