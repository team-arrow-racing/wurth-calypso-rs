use atat::Buffers;

use crate::command::Urc;

// todo: these need to be configurable
pub const INGRESS_BUF_SIZE: usize = 1024;
pub const URC_CAPACITY: usize = 128;
pub const URC_SUBSCRIBERS: usize = 3;

pub static BUFFERS: Buffers<
    Urc,
    INGRESS_BUF_SIZE,
    URC_CAPACITY,
    URC_SUBSCRIBERS,
> = Buffers::<Urc, INGRESS_BUF_SIZE, URC_CAPACITY, URC_SUBSCRIBERS>::new();
