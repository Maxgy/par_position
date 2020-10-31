use std::{
    sync::{mpsc, Arc},
    thread,
};

pub fn par_pos<T>(v: Arc<Vec<T>>, val: T) -> Option<usize>
where
    T: 'static + Copy + PartialEq + Send + Sync,
{
    par_pos_with_num_threads(v, val, 4)
}

pub fn par_pos_with_num_threads<T>(v: Arc<Vec<T>>, val: T, num_threads: usize) -> Option<usize>
where
    T: 'static + Copy + PartialEq + Send + Sync,
{
    if num_threads == 0 {
        None
    } else if num_threads == 1 {
        v.iter().position(|&x| x == val)
    } else {
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
                if let Some(x) = found {
                    tx1.send(Some(x + n * chunk_size)).ok();
                } else {
                    tx1.send(None).ok();
                }
            });
            handles.push(t);
        }

        let mut num_recv = 0;
        while num_recv < num_threads {
            if let Ok(recv) = rx.try_recv() {
                if let Some(x) = recv {
                    for t in handles {
                        t.join().unwrap();
                    }
                    return Some(x);
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
