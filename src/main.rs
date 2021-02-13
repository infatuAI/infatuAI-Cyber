extern crate pcap;
extern crate argparse;
extern crate figlet_rs;

use argparse::{ArgumentParser, Store, StoreTrue};
use pcap::{Capture, Device};
use figlet_rs::{FIGfont};

fn main() {

    let devices = Device::list();
    for device in devices {
        println!("\t* Device found {:?}", device);
    }

    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert("infatuAI Cyber");
    println!("{}", figure.unwrap());

}
