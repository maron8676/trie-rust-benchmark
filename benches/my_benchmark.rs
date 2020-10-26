extern crate criterion;

extern crate radix_trie;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use radix_trie::Trie;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn bench_search(c: &mut Criterion) {
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

    let word = "べんちまーく";
    let mut search_group = c.benchmark_group("Search");
    search_group.bench_with_input(BenchmarkId::new("Binary", 0), word, |b, word| {
        b.iter(|| sorted.binary_search(&word.to_string()))
    });
    search_group.bench_with_input(BenchmarkId::new("HashMap", 0), word, |b, word| {
        b.iter(|| hashmap.get(&word))
    });
    search_group.bench_with_input(BenchmarkId::new("Trie", 0), word, |b, word| {
        b.iter(|| trie.get(&word))
    });
    search_group.finish();

    let sentence = "わかちがきひかくべんちまーく";
    let mut separation_group = c.benchmark_group("Separation");
    separation_group.bench_with_input(BenchmarkId::new("Binary", 0), sentence, |b, word| {
        b.iter(|| {
            let chars = word.chars().collect::<Vec<_>>();
            let mut start_index = 0;
            let mut end_index = start_index + 3;
            let mut separated = vec![];

            while end_index <= sentence.len() {
                for index in (start_index + 3..sentence.len() + 1).step_by(3) {
                    if let Ok(_) = sorted.binary_search(&sentence[start_index..index].to_string()) {
                        end_index = index;
                    }
                }
                separated.push(
                    chars[start_index / 3..end_index / 3]
                        .into_iter()
                        .collect::<String>(),
                );
                start_index = end_index;
                end_index = start_index + 3;
            }

            separated
        })
    });
    separation_group.bench_with_input(BenchmarkId::new("HashMap", 0), sentence, |b, word| {
        b.iter(|| {
            let chars = word.chars().collect::<Vec<_>>();
            let mut start_index = 0;
            let mut end_index = start_index + 3;
            let mut separated = vec![];

            while end_index <= sentence.len() {
                for index in (start_index + 3..sentence.len() + 1).step_by(3) {
                    if let Some(_) = hashmap.get(&sentence[start_index..index]) {
                        end_index = index;
                    }
                }
                separated.push(
                    chars[start_index / 3..end_index / 3]
                        .into_iter()
                        .collect::<String>(),
                );
                start_index = end_index;
                end_index = start_index + 3;
            }

            separated
        })
    });
    separation_group.bench_with_input(BenchmarkId::new("Trie", 0), sentence, |b, word| {
        b.iter(|| {
            let chars = word.chars().collect::<Vec<_>>();
            let mut start_index = 0;
            let mut end_index = start_index + 3;
            let mut separated = vec![];

            while end_index <= chars.len() {
                if let Some(word) =
                    trie.get_ancestor_value(&sentence[start_index..])
                {
                    end_index = start_index + word.len();
                }
                separated.push(
                    chars[start_index/3..end_index/3]
                        .into_iter()
                        .collect::<String>(),
                );
                start_index = end_index;
                end_index = start_index + 3;
            }

            separated
        })
    });
    separation_group.finish();
}

criterion_group!(benches, bench_search);
criterion_main!(benches);
