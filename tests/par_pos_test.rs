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
    }

    #[test]
    fn threaded_test() {
        let mut v = vec![0; 10_000_000];
        v[5_999_999] = 42;
        let v1 = Arc::new(v.clone());

        let t2 = thread::spawn(move || {
            v.iter().position(|&x| x == 42);
        });
        let t1 = thread::spawn(move || {
            par_pos(v1.clone(), 42);
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
}
