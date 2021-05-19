#![no_std]

use uavcan::tx::breakdown::Breakdown;
use uavcan::{CanFrame, CLASSIC_MTU};

#[derive(Debug)]
struct Frame {
    id: u32,
    data: [u8; 8],
    len: usize,
}

impl From<(u32, [u8; 8], usize)> for Frame {
    fn from((id, data, len): (u32, [u8; 8], usize)) -> Self {
        Self { data, id, len }
    }
}

impl CanFrame<CLASSIC_MTU> for Frame {
    fn id(&self) -> u32 {
        self.id
    }

    fn payload(&self) -> (&[u8; CLASSIC_MTU], usize) {
        (&self.data, self.len)
    }
}

pub struct UavcanFrames<'a> {
    frames: Breakdown<'a, Frame, CLASSIC_MTU>,
}

impl<'a> Iterator for UavcanFrames<'a> {
    type Item = ([u8; CLASSIC_MTU], usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.frames.next().map(|frame| (frame.data, frame.len))
    }
}

pub fn to_uavcan(payload: &[u8]) -> UavcanFrames {
    UavcanFrames {
        // We don't really use the frames that are returned so the can_id can be
        // ignored here.
        frames: Breakdown::new(payload, 0),
    }
}
