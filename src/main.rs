enum SearchError {
    NotFound
}

fn search(v: &[usize], n: usize, i: usize) -> Result<usize, SearchError> {
    let len = v.len();
    let mid = len / 2;

    if n == v[mid] {
        return Ok(i + mid);
    }

    if len <= 1 {
        return Err(SearchError::NotFound);
    }

    if n < v[mid] {
        return search(&v[0..mid], n, i);
    } else {
        return search(&v[mid..len], n, i + mid);
    }
}

fn bsearch(v: &[usize], n: i32) -> Result<usize, SearchError> {
    let n = n as usize;
    search(v, n, 0)
}

fn main() {
    let mut v = vec![0; 10000];
    for n in 0..10000 {
        v[n] = n;
    }
    let s = &v[..];

    match bsearch(s, 567) {
        Ok(index) => println!("Match found at index: {}", index),
        Err(_) => println!("Failed to find match")
    }
}
