pub fn pl(msg: &str) {
    println!("{}", msg);
}

pub fn p(msg: &str) {
    print!("{}", msg);
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}