extern crate futures;

use std::sync::mpsc::channel;

use futures::*;

#[test]
fn lots() {
    fn doit(n: usize) -> Box<Future<Item=(), Error=()> + Send> {
        if n == 0 {
            finished(()).boxed_send()
        } else {
            finished(n - 1).and_then(doit).boxed_send()
        }
    }

    let (tx, rx) = channel();
    doit(1_000).map(move |_| tx.send(()).unwrap()).forget();
    rx.recv().unwrap();
}
