use crc_any::CRCu16;
use uavcan_bridge::to_uavcan;

fn main() {
    let fascism_reborn = "Hello world! Hello world! Greet back you unloyal scum!".as_bytes();
    let mut crc = CRCu16::crc16ccitt_false();
    crc.digest(fascism_reborn);

    println!("Data is: {:?}", fascism_reborn);
    println!("Crc is: {:?}\n", crc.get_crc().to_be_bytes());

    to_uavcan(fascism_reborn).for_each(|data| println!("{:?}", data));
}
