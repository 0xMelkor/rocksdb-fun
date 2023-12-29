use reth_libmdbx::{Environment, Geometry, DatabaseFlags, WriteFlags};
use rocksdb::{Options, DB};
use std::{env, thread::sleep, time::Duration, path::Path};

fn main() {
    const HEIGHT_KEY: [u8; 1] = [0];
    let path = env::var("MDBX_PATH").expect("MDBX_PATH env");
    let path = Path::new(&path);

    let env = {
        let mut builder = Environment::builder();
        builder.set_max_dbs(2);
        builder
            .set_geometry(Geometry { size: Some(1_000_000..1_000_000), ..Default::default() });
        builder.open(path).expect("open mdbx env")
    };

    for _height in 0..1000 {
        let mut value = [0u8; 8];
        let tx = env.begin_rw_txn().expect("begin_rw_txn");
        let index = tx.create_db(None, DatabaseFlags::DUP_SORT).expect("open index db");
        tx.put(index.dbi(), HEIGHT_KEY, value, WriteFlags::empty()).expect("tx.put");
        tx.commit().expect("tx.commit");
    }
}
