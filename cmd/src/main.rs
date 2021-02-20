use std::env;
use std::fs::File;

use libmagie::codec::bitmap;

fn main() {
    println!("{}", env::current_dir().unwrap().display());

    let mut file = match File::open("../images/rgb.bmp") {
        Err(e) => panic!(e),
        Ok(file) => file,
    };

    let _ = bitmap::decoder::decode(&mut file);
}
