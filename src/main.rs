mod bsearch;

fn main() {
    let mut v = vec![0; 10000];
    for n in 0..10000 {
        v[n] = n;
    }
    let s = &v[..];

    match bsearch::bsearch(s, 567) {
        Ok(index) => println!("Match found at index: {}", index),
        Err(_) => println!("Failed to find match")
    }
}
