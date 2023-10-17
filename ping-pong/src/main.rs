const N: u64 = 1_000_000;

use crossbeam_channel::{Receiver, Sender};

pub fn main() {
    let t = std::time::Instant::now();
    let (a_to_b, b_from_a) = crossbeam_channel::bounded(0);
    let (b_to_a, a_from_b) = crossbeam_channel::bounded(0);
    std::thread::scope(|scope| {
        scope.spawn(move || worker(true, a_from_b, a_to_b));
        scope.spawn(move || worker(false, b_from_a, b_to_a));
    });
    eprintln!("{:?}", t.elapsed());
}

fn worker(init: bool, receiver: Receiver<u64>, sender: Sender<u64>) {
    if init {
        sender.send(0).unwrap();
    }
    for x in receiver {
        if x == N {
            break;
        }
        sender.send(x + 1).unwrap();
    }
}
