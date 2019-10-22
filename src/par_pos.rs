use std::{
    sync::{mpsc, Arc},
    thread,
};

pub fn par_pos<T: 'static + Copy + PartialEq + Send + Sync>(
    v: Arc<Vec<T>>,
    val: T,
) -> Option<usize> {
    let num_threads = 2;

    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::new();

    let chunk_size = v.len() / num_threads;

    for n in 0..num_threads {
        let v1 = v.clone();
        let tx1 = tx.clone();

        let t = thread::spawn(move || {
            let a = n * chunk_size;
            let b = (a as f64 + chunk_size as f64).ceil() as usize;

            let found = v1[a..b].iter().position(|&x| x == val);
            if tx1.send((n, found)).is_ok() {}
        });
        handles.push(t);
    }

    let mut num_recv = 0;
    while num_recv < 2 {
        if let Ok(recv) = rx.try_recv() {
            if let (n, Some(x)) = recv {
                for t in handles {
                    t.join().unwrap();
                }
                return Some(x + n * chunk_size);
            } else {
                num_recv += 1;
            }
        }
    }

    for t in handles {
        t.join().unwrap();
    }
    None
}
