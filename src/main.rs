use std::sync::Arc;
use std::thread;

use par_position::*;

fn main() {
    // Start a couple of threads for par_pos, par_pos_with_num_threads, and .iter().position()
    //
    // The first to find the value will print first

    // par_pos first
    let mut v = vec![0; 10_000_000];
    v[9_999_999] = 42;
    let v1 = Arc::new(v.clone());
    let v2 = v1.clone();

    let t3 = thread::spawn(move || {
        println!("0 par_pos:  {:?}", par_pos(v1.clone(), 42));
    });
    let t1 = thread::spawn(move || {
        println!("0 _par_pos: {:?}", par_pos_with_num_threads(v2, 42, 7));
    });
    let t2 = thread::spawn(move || {
        println!("0 iter pos: {:?}", v.iter().position(|&x| x == 42));
    });

    t2.join().unwrap();
    t1.join().unwrap();
    t3.join().unwrap();
    println!();

    // iter pos first
    let mut v = vec![0; 10_000_000];
    v[0] = 42;
    v[9_999_999] = 42;
    let v1 = Arc::new(v.clone());
    let v2 = v1.clone();

    let t2 = thread::spawn(move || {
        println!("1 iter pos: {:?}", v.iter().position(|&x| x == 42));
    });
    let t3 = thread::spawn(move || {
        println!("1 par_pos:  {:?}", par_pos(v1.clone(), 42));
    });
    let t1 = thread::spawn(move || {
        println!("1 _par_pos: {:?}", par_pos_with_num_threads(v2, 42, 7));
    });

    t2.join().unwrap();
    t1.join().unwrap();
    t3.join().unwrap();
    println!();

    // other type of value
    let mut v = vec!['a'; 10_000_000];
    v[10_000_000 / 4 * 3] = 'b';
    let v1 = Arc::new(v.clone());
    let v2 = v1.clone();

    let t3 = thread::spawn(move || {
        println!("2 par_pos:  {:?}", par_pos(v1.clone(), 'b'));
    });
    let t1 = thread::spawn(move || {
        println!("2 _par_pos: {:?}", par_pos_with_num_threads(v2, 'b', 4));
    });
    let t2 = thread::spawn(move || {
        println!("2 iter pos: {:?}", v.iter().position(|&x| x == 'b'));
    });

    t2.join().unwrap();
    t1.join().unwrap();
    t3.join().unwrap();
    println!();

    // does not exist
    let mut v = vec![0; 10_000_000];
    v[5_999_999] = 42;
    let v1 = Arc::new(v.clone());
    let v2 = v1.clone();

    let t2 = thread::spawn(move || {
        println!("3 iter pos: {:?}", v.iter().position(|&x| x == 4));
    });
    let t3 = thread::spawn(move || {
        println!("3 par_pos:  {:?}", par_pos(v1.clone(), 4));
    });
    let t1 = thread::spawn(move || {
        println!("3 _par_pos: {:?}", par_pos_with_num_threads(v2, 4, 7));
    });

    t1.join().unwrap();
    t3.join().unwrap();
    t2.join().unwrap();
}
