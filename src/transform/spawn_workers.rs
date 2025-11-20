use flume::{Receiver, Sender};
use rayon::prelude::*;
use std::thread;
use crate::rustle::{extract};

pub fn spawn_workers(
    rx_raw: Receiver<extract::RawRecord>,
    tx_trans: Sender<super::TransformedRecord>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        rx_raw
            .into_iter()
            .par_bridge()
            .for_each(|rec| {
                if let Some(t) = super::transform(rec) {
                    match tx_trans.send(t) {
                        Err(x) => println!("A transform error occurred: {}", x),
                        _ => (),
                    };
                }
            });
        drop(tx_trans);
    })
}
