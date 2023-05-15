use ahash::AHashMap;

fn main() {
    let pattern: &[u8] = &['w' as u8, 'a' as u8, 'l' as u8, 'k' as u8];
    let text = String::from("Will you walk into my parlour? said a spider to a fly");
    let text_bytes = text.as_bytes();

    let bad_match_table = pre_process_pattern(&pattern);

    let result = search(&bad_match_table, &pattern, &text_bytes);
    println!("The result is: {:?}", result.unwrap());
}

fn pre_process_pattern(pattern: &[u8]) -> AHashMap<u8, usize> {
    let mut bad_match_table: AHashMap<u8, usize> = AHashMap::new();

    for (index, letter) in pattern.iter().enumerate() {
        bad_match_table.insert(
            *letter as u8,
            pattern.len() - index - 1,
        );
    }

    bad_match_table.insert('*' as u8, pattern.len());

    return bad_match_table;
}

fn search(bad_match_table: &AHashMap<u8, usize>, pattern: &[u8], text: &[u8]) -> Option<usize> {
    let mut index = pattern.len() - 1;
    let pattern_len = pattern.len();
    let one = 1;

    while index < text.len() {
        for pattern_index in (0..pattern.len()).rev() {
            if text[index] != pattern[pattern_index] {
                let shift = bad_match_table.get(&text[index]).unwrap_or(&pattern_len);
                index += shift.max(&one);
                break;
            } else if pattern_index == 0 {
                return Some(index);
            }
            index -= 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let pattern: &[u8] = &['w' as u8, 'a' as u8, 'l' as u8, 'k' as u8];
        let text = String::from("Will you walk into my parlour? said a spider to a fly");
        let text_bytes = text.as_bytes();

        let bad_match_table = pre_process_pattern(&pattern);

        assert_eq!(search(&bad_match_table, &pattern, &text_bytes), Some(9));
    }

    #[test]
    fn test_search_not_found() {
        let pattern: &[u8] = &['z' as u8, 'z' as u8, 'z' as u8];
        let text = String::from("Will you walk into my parlour? said a spider to a fly");
        let text_bytes = text.as_bytes();

        let bad_match_table = pre_process_pattern(&pattern);

        assert_eq!(search(&bad_match_table, &pattern, &text_bytes), None);
    }
}
