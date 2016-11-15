pub enum SearchError {
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

pub fn bsearch(v: &[usize], n: i32) -> Result<usize, SearchError> {
    let n = n as usize;
    search(v, n, 0)
}
