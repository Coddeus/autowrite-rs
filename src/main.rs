extern crate clipboard;
extern crate enigo;

use std::fs::File;
use std::io::Read;

use enigo::*;

fn main() {
    let mut enigo = Enigo::new();
    let mut whole = String::new();
    File::open(".readme").unwrap().read_to_string(&mut whole).unwrap();

    std::thread::sleep(std::time::Duration::new(2, 0));

    for line in whole.split("\n").into_iter() {
        enigo.key_sequence(&line.to_string());
        enigo.key_click(Key::Return);
        enigo.key_click(Key::Space);
        enigo.key_down(Key::LControl);
        enigo.key_click(Key::LeftArrow);
        enigo.key_click(Key::Delete);
        enigo.key_click(Key::Delete);
        enigo.key_up(Key::LControl);
    }
}