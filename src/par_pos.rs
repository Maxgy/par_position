use std::{
    sync::{mpsc, Arc},
    thread,
};

pub fn par_pos<T: 'static + Copy + PartialEq + Send + Sync>(v: Arc<Vec<T>>, val: T) -> Option<usize> {
    let num_threads = 2;

    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::new();

    for n in 0..num_threads {
        let v1 = v.clone();
        let tx1 = tx.clone();

        let t = thread::spawn(move || {
            let a = n * (v1.len() / num_threads);
            let b = (a as f64 + v1.len() as f64 / num_threads as f64).ceil() as usize;

            let found = v1[a..b].iter().position(|&x| x == val);
            tx1.send((n, found)).unwrap();
        });
        handles.push(t);
    }

    for t in handles {
        t.join().unwrap();
    }

    for recv in rx.try_iter() {
        if let (n, Some(x)) = recv {
            return Some(x + n * (v.len() / num_threads));
        }
    }
    None
}
