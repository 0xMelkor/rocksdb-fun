use std::{env, path::Path};

use reth_db::open_db_read_only;

fn main() {
    let path = env::var("MDBX_PATH").expect("MDBX_PATH env");
    let path = Path::new(&path);
    if let Err(e) = open_db_read_only(&path, None) {
        println!("{e:?}");
    }
}
