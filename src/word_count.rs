use std::collections::HashMap;

pub fn count_word(contents: &str) -> Vec<(String, u32)> {
    let mut wordcount = HashMap::new();

    for s in contents.split_whitespace() {
        let c = wordcount.entry(s).or_insert(0);
        *c += 1;
    }

    let mut wordcount: Vec<(&str, u32)> = wordcount.into_iter().collect();
    wordcount.sort_by(|&a, &b| b.1.cmp(&a.1));

    return wordcount
        .iter()
        .map(|&(w, count)| (w.to_owned(), count))
        .collect();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn count_word_test() {
        let contents = fs::read_to_string("src/word_count.rs").expect("file not found");

        let mut wordcount = HashMap::new();

        for s in contents.split_whitespace() {
            let c = wordcount.entry(s).or_insert(0);
            *c += 1;
        }

        let mut wordcount: Vec<(&&str, &u32)> = wordcount.iter().collect();
        wordcount.sort_by(|a, b| b.1.cmp(a.1));

        for (w, count) in wordcount.iter().take(5) {
            println!("{}: {}", w, count);
        }
    }

    #[test]
    fn count_word_fn_test() {
        let contents = fs::read_to_string("src/word_count.rs").expect("file not found");

        for (w, count) in count_word(&contents).iter().take(5) {
            println!("{}: {}", w, count);
        }
    }
}
