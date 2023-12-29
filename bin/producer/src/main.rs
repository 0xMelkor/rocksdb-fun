use std::{env, thread::sleep, time::Duration};

use rocksdb::{DB, Options};

fn main() {
    let path = env::var("ROCKSDB_PATH").expect("ROCKSDB_PATH env");

    let mut opts = Options::default();
    opts.set_write_buffer_size(128 * 1024);
    
    // NB: db is automatically closed at end of lifetime
    let db = DB::open(&opts, path).unwrap();

    let mut num: u8  = 0;
    loop {
        println!("{num}");
        let bytes = [num];
        db.put(b"my key", bytes).expect("write");
        num = (num + 1) % 255;
        sleep(Duration::from_millis(10));
    }
}