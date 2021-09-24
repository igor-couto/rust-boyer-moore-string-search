use ahash::AHashMap;

fn main() {
    let pattern: &[u8] = &['w' as u8, 'a' as u8, 'l' as u8, 'k' as u8];
    let text = String::from("Will you walk into my parlour? said a spider to a fly").as_bytes();

    let bad_match_table = pre_process_pattern(&pattern);
}

fn pre_process_pattern(pattern: &[u8]) -> AHashMap<u8, u8> {
    let mut bad_match_table: AHashMap<u8, u8> = AHashMap::new();

    for (index, letter) in pattern.iter().enumerate() {
        bad_match_table.insert(
            *letter as u8,
            max(1, (pattern.len() - index - 1usize) as u8),
        );
    }

    bad_match_table.insert('*' as u8, pattern.len() as u8);

    return bad_match_table;
}

fn max(n1: u8, n2: u8) -> u8 {
    if n1 > n2 {
        n1
    } else {
        n2
    }
}
