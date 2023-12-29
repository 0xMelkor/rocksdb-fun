use rocksdb::{Options, DB};
use std::{env, thread::sleep, time::Duration};

fn main() {
    loop {
        let path = env::var("ROCKSDB_PATH").expect("ROCKSDB_PATH env");
        let db = DB::open_for_read_only(&Options::default(), path, false).unwrap();

        match db.get(b"my key") {
            Ok(Some(value)) => println!("retrieved value {:?}", value),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }

        sleep(Duration::from_millis(100));
    }
}
