use ahash::AHashMap;

fn main() {
    let pattern: &[u8] = &['w' as u8, 'a' as u8, 'l' as u8, 'k' as u8];
    let text = String::from("Will you walk into my parlour? said a spider to a fly");
    let text_bytes = text.as_bytes();

    let bad_match_table = pre_process_pattern(&pattern);

    let result = search(&bad_match_table, &pattern, &text_bytes);
    println!("The result is: {}", result);
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

fn search(bad_match_table: &AHashMap<u8, u8>, pattern: &[u8], text: &[u8]) -> u32 {
    let mut index = (bad_match_table.get(&('*' as u8)).unwrap() - 1u8) as usize;
    'text_loop: loop {
        if index >= text.len() {
            // TODO : return error not found here
            break;
        }

        for pattern_index in (0..pattern.len()).rev() {
            if text[index] != pattern[pattern_index] {
                index += match bad_match_table.get(&text[index]) {
                    Some(value) => *value as usize,
                    None => *bad_match_table.get(&('*' as u8)).unwrap() as usize,
                };
                continue 'text_loop;
            }
            return (index - pattern.len()) as u32;
        }

        index += 1;
    }
    return index as u32;
}

fn max(n1: u8, n2: u8) -> u8 {
    if n1 > n2 {
        n1
    } else {
        n2
    }
}
