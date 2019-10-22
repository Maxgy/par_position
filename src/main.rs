use std::sync::Arc;
use std::thread;

use par_position::*;

fn main() {
    // par_pos first
    let mut v = vec![0; 10_000_000];
    v[9_999_999] = 42;
    let v1 = Arc::new(v.clone());

    let t1 = thread::spawn(move || {
        println!("0 par_pos:  {:?}", par_pos(v1.clone(), 42));
    });
    let t2 = thread::spawn(move || {
        println!("0 iter pos: {:?}", v.iter().position(|&x| x == 42));
    });

    t2.join().unwrap();
    t1.join().unwrap();

    // iter pos first
    let mut v = vec![0; 10_000_000];
    v[5_999_999] = 42;
    let v1 = Arc::new(v.clone());

    let t2 = thread::spawn(move || {
        println!("1 iter pos: {:?}", v.iter().position(|&x| x == 42));
    });
    let t1 = thread::spawn(move || {
        println!("1 par_pos:  {:?}", par_pos(v1.clone(), 42));
    });

    t1.join().unwrap();
    t2.join().unwrap();

    // does not exist
    let mut v = vec![0; 10_000_000];
    v[5_999_999] = 42;
    let v1 = Arc::new(v.clone());

    let t2 = thread::spawn(move || {
        println!("2 iter pos: {:?}", v.iter().position(|&x| x == 4));
    });
    let t1 = thread::spawn(move || {
        println!("2 par_pos:  {:?}", par_pos(v1.clone(), 4));
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
