fn build_table(n: i32, tab: &mut [u16], descending: bool) {
    for i in 0..n {
        let v = (65535.0 * i as f64) / (n-1) as f64;

        tab[(if descending {n - i - 1} else {i}) as usize] = (v + 0.5).floor() as u16;
    }
}
