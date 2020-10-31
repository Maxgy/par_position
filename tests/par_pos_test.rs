#[cfg(test)]
mod test {
    use std::{sync::Arc, thread};

    use par_position::*;

    #[test]
    fn par_pos_test() {
        let v = Arc::new(vec![
            6, 1, 7, 12, 87, 2, 50, 77, 3, 5, 42, 65, 22, 68, 222, 0,
        ]);

        for val in v.iter() {
            assert_eq!(par_pos(v.clone(), *val), v.iter().position(|x| x == val));
        }

        assert_eq!(par_pos(v.clone(), 6), v.iter().position(|&x| x == 6));
        assert_eq!(par_pos(v.clone(), 87), v.iter().position(|&x| x == 87));
        assert_eq!(par_pos(v.clone(), 42), v.iter().position(|&x| x == 42));
        assert_eq!(par_pos(v.clone(), 222), v.iter().position(|&x| x == 222));
        assert_eq!(par_pos(v.clone(), 0), v.iter().position(|&x| x == 0));
        assert_eq!(par_pos(v.clone(), 1000), v.iter().position(|&x| x == 1000));
    }

    #[test]
    fn par_pos_test_large() {
        let mut v = vec![0; 50_000_000];
        v[300] = 42;
        v[500] = 420;
        v[750] = 66;
        v[1_007_000] = 69;
        let v = Arc::new(v);

        assert_eq!(par_pos(v.clone(), 42), v.iter().position(|&x| x == 42));
        assert_eq!(par_pos(v.clone(), 420), v.iter().position(|&x| x == 420));
        assert_eq!(par_pos(v.clone(), 66), v.iter().position(|&x| x == 66));
        assert_eq!(par_pos(v.clone(), 69), v.iter().position(|&x| x == 69));
        assert_eq!(
            par_pos_with_num_threads(v.clone(), 69, 3),
            v.iter().position(|&x| x == 69)
        );
        assert_eq!(
            par_pos_with_num_threads(v.clone(), 69, 4),
            v.iter().position(|&x| x == 69)
        );
        assert_eq!(
            par_pos_with_num_threads(v.clone(), 69, 5),
            v.iter().position(|&x| x == 69)
        );
        assert_eq!(
            par_pos_with_num_threads(v.clone(), 69, 6),
            v.iter().position(|&x| x == 69)
        );
        assert_eq!(
            par_pos_with_num_threads(v.clone(), 69, 7),
            v.iter().position(|&x| x == 69)
        );
    }

    #[test]
    fn threaded_test() {
        let mut v = vec!['a'; 10_000_000];
        v[5_999_999] = 'b';
        let v1 = Arc::new(v.clone());

        let t2 = thread::spawn(move || {
            v.iter().position(|&x| x == 'b');
        });
        let t1 = thread::spawn(move || {
            par_pos(v1.clone(), 'b');
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }

    #[test]
    fn examples() {
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
}
