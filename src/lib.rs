fn search(v: &[i32], n: i32, i: usize) -> Option<usize> {
    if v.is_empty() {
        return None;
    };

    let len = v.len();
    let mid = len / 2;

    if n == v[mid] {
        return Some(i + mid);
    }

    if len <= 1 {
        return None;
    }

    if n < v[mid] {
        return search(&v[0..mid], n, i);
    } else {
        return search(&v[mid..len], n, i + mid);
    }
}

pub fn bsearch(v: &[i32], n: i32) -> Option<usize> {
    search(v, n, 0)
}
