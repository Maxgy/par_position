use std::{
    sync::{mpsc, Arc},
    thread,
};

pub fn par_pos<T: 'static + Copy + PartialEq + Send + Sync>(
    v: Arc<Vec<T>>,
    val: T,
) -> Option<usize> {
    par_pos_with_num_threads(v, val, 2)
}

pub fn par_pos_with_num_threads<T: 'static + Copy + PartialEq + Send + Sync>(
    v: Arc<Vec<T>>,
    val: T,
    num_threads: usize,
) -> Option<usize> {
    if num_threads == 0 {
        None
    } else if num_threads == 1 {
        v.iter().position(|&x| x == val)
    } else {
        let num_threads = if num_threads >= 100 { 100 } else { num_threads };

        let (tx, rx) = mpsc::channel();

        let mut handles = Vec::new();

        let chunk_size = v.len() / num_threads;

        for n in 0..num_threads {
            let v1 = v.clone();
            let tx1 = tx.clone();

            let t = thread::spawn(move || {
                let a = n * chunk_size;
                let b = if n + 1 == num_threads {
                    v1.len()
                } else {
                    a + chunk_size
                };

                let found = v1[a..b].iter().position(|&x| x == val);
                if tx1.send((n, found)).is_ok() {}
            });
            handles.push(t);
        }

        let mut num_recv = 0;
        while num_recv < num_threads {
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
}
