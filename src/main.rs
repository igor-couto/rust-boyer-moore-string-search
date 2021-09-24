use std::vec;

fn main() {
    let pattern: &[u8] = &['w' as u8, 'a' as u8, 'l' as u8, 'k' as u8];
    let text = String::from("Will you walk into my parlour? said a spider to a fly").as_bytes();
}

fn max(n1: u8, n2: u8) -> u8 {
    if n1 > n2 {
        n1
    } else {
        n2
    }
}
