use std::sync::Arc;

use par_position::par_pos;

fn main() {
    let v = Arc::new(vec![
        6, 1, 7, 12, 87, 2, 50, 77, 3, 5, 42, 65, 22, 68, 222, 0,
    ]);

    println!(
        "{:?} vs {:?}\n\
         {:?} vs {:?}\n\
         {:?} vs {:?}\n\
         {:?} vs {:?}",
        par_pos(v.clone(), 6),
        v.iter().position(|&x| x == 6),
        par_pos(v.clone(), 87),
        v.iter().position(|&x| x == 87),
        par_pos(v.clone(), 42),
        v.iter().position(|&x| x == 42),
        par_pos(v.clone(), 0),
        v.iter().position(|&x| x == 0),
    );
}
