extern crate radix_trie;

use radix_trie::Trie;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let mut sorted = vec![];
    let mut trie = Trie::<String, String>::new();
    let mut hashmap = HashMap::<String, String>::new();

    let lines = {
        let file = File::open("data/word.txt").expect("failed to open file");
        BufReader::new(file).lines()
    };
    for line in lines {
        if let Ok(word) = line {
            if let Err(index) = sorted.binary_search(&word) {
                sorted.insert(index, word.clone());
            };
            trie.insert(word.clone(), word.clone());
            hashmap.insert(word.clone(), word.clone());
        }
    }

    println!("{:#?}", trie.get_ancestor_value("べんちまーく"));
    println!("{:#?}", hashmap.get("べんちまーく"));

    let word = "わかちがきひかくべんちまーくあうとぷっと";

    let chars = word.chars().collect::<Vec<_>>();
    let mut start_index = 0;
    let mut end_index = start_index + 1;
    let mut separated = vec![];

    while end_index <= chars.len() {
        for index in start_index + 1..chars.len() + 1 {
            if let Some(_) = hashmap.get(&chars[start_index..index].into_iter().collect::<String>())
            {
                end_index = index;
            }
        }
        separated.push(
            chars[start_index..end_index]
                .into_iter()
                .collect::<String>(),
        );
        start_index = end_index;
        end_index = start_index + 1;
    }
    println!("{:#?}", separated);

    let chars = word.chars().collect::<Vec<_>>();
    let mut start_index = 0;
    let mut end_index = start_index + 1;
    let mut separated = vec![];

    while end_index <= chars.len() {
        if let Some(word) =
            trie.get_ancestor_value(&chars[start_index..].into_iter().collect::<String>())
        {
            end_index = start_index + word.chars().count();
        }
        separated.push(
            chars[start_index..end_index]
                .into_iter()
                .collect::<String>(),
        );
        start_index = end_index;
        end_index = start_index + 1;
    }
    println!("{:#?}", separated);

    Ok(())
}
