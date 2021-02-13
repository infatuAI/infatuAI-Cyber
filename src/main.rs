extern crate pcap;
extern crate argparse;
extern crate figlet_rs;

use argparse::{ArgumentParser, Store, StoreTrue};
use pcap::{Capture, Device};
use figlet_rs::{FIGfont};

fn main() {

    let devices = Device::list();
    if let device = devices {
        println!("\t* Device found {:?}", device);
    }

    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert("infatuAI Cyber");
    println!("{}", figure.unwrap());

    let main_device = Device::lookup().unwrap();
    println!("\t* Using device {:?}", main_device);

    let mut cap = Capture::from_device(main_device).unwrap()
    .promisc(true)
    .open().unwrap();

    while let Ok(packet) = cap.next() {
        println!("received packet! {:?}", packet);
    }

}
