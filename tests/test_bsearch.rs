extern crate bsearch;

#[test]
fn find_indices() {
    let mut v = vec![0; 100];
    for n in 0..100 {
        v[n] = n as i32;
    }

    for (expected_index, n) in (0..100).enumerate() {
        let index = match bsearch::bsearch(&v, n) {
            Some(i) => i,
            None => 1,
        };
        assert_eq!(index, expected_index as usize);
    }
}

#[test]
fn find_indices_negative_values() {
    let v = vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5];

    for (expected_index, n) in (&v).iter().enumerate() {
        let index = match bsearch::bsearch(&v, *n) {
            Some(i) => i,
            None => 1,
        };
        assert_eq!(index, expected_index);
    }
}

#[test]
fn empty_vector() {
    let v = vec![];
    let n = 5;

    let index = match bsearch::bsearch(&v, n) {
        Some(i) => i,
        None => 1,
    };
    assert_eq!(index, 1);
}
