use rand::Rng;
use std::sync::Arc;
use std::sync::Mutex;
use std::{thread, time};

fn main() {
    let mut handles = vec![];
    let finished_counter = Arc::new(Mutex::new(0));

    for i in 0..10 {
        let counter = Arc::clone(&finished_counter);
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let millisecond = rng.gen_range(500..=5000);
            thread::sleep(time::Duration::from_millis(millisecond));

            let mut num = counter.lock().unwrap();
            *num += 1;

            println!("{}: finished", i);
        });
        handles.push(handle);
    }

    // 各スレッドの終了を待つ
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", finished_counter.lock().unwrap());
}
