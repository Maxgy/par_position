use std::{sync::Arc, thread};

use rayon::prelude::*;

use par_position::*;

fn main() {
    // let mut v = vec![0; 10_000_000];
    let mut v = vec![0; 1_100_000_000];
    let n = 42;
    v[669_690_069] = n;
    let v1 = Arc::new(v.clone());
    let v2 = v1.clone();
    let v3 = v1.clone();
    let v4 = v1.clone();
    let v5 = v1.clone();

    let t2 = thread::spawn(move || {
        println!("iter.pos: {:?}", v.iter().position(|&x| x == n));
    });
    let t3 = thread::spawn(move || {
        println!("par_pos:  {:?}", par_pos(v1, n));
    });
    let t6 = thread::spawn(move || {
        println!("par_iter: {:?}", v5.par_iter().position_any(|&x| x == n));
    });
    let t1 = thread::spawn(move || {
        println!("_par_pos: {:?}", par_pos_with_num_threads(v2, n, 512 - 256));
    });
    let t4 = thread::spawn(move || {
        println!("_ par_pos:{:?}", par_pos_with_num_threads(v3, n, 512 - 128));
    });
    let t5 = thread::spawn(move || {
        println!("__par_pos:{:?}", par_pos_with_num_threads(v4, n, 512));
    });

    t5.join().unwrap();
    t4.join().unwrap();
    t1.join().unwrap();
    t6.join().unwrap();
    t3.join().unwrap();
    t2.join().unwrap();
}
