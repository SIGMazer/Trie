use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::env;


#[derive(Clone)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            end: false,
        }
    }
    fn insert(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new());
        }
        node.end = true;
    }
    fn complete(&self, prefix: String) -> Vec<String> {
        let mut completions = Vec::new();
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return completions,
            }
        }
        node.complete_helper(prefix, &mut completions);
        completions
    }
    fn complete_helper(&self, prefix: String, completions: &mut Vec<String>) {
        if self.end {
            completions.push(prefix.clone());
        }
        for (c, n) in &self.children {
            let mut new_prefix = prefix.clone();
            new_prefix.push(*c);
            n.complete_helper(new_prefix, completions);
        }
    }
    fn dump_dot<W: Write>(&self, sink: &mut W, node_pool: &mut Vec<TrieNode>, index: usize) {
        writeln!(sink, "    Node_{} [label=\"{}\"]", index, if self.end { '*' } else { ' ' }).unwrap();
        for (_i, (c, child)) in self.children.iter().enumerate() {
            let child_index = node_pool.len();
            node_pool.push(child.clone());
            writeln!(sink, "    Node_{} -> Node_{} [label=\"{}\"]", index, child_index, c).unwrap();
            child.dump_dot(sink, node_pool, child_index);
        }
    }
}

fn read_file(file_name: String) -> Vec<String> {
    let mut words = Vec::new();
    let file = File::open(file_name).expect("file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        words.push(line.unwrap());
    }
    words
}

fn usage() {
    println!("Usage: trie <command> [<args>]");
    println!("Commands:");
    println!("  dot: dump the trie in dot format");
    println!("  complete <prefix> : complete the prefix");
}


fn main(){
    let mut trie = TrieNode::new();
    let args: Vec<String> = env::args().collect();
    let words = read_file("words.txt".to_string());
    for word in words {
        trie.insert(word);
    }
    if args.len() > 1 {
        if args[1] == "dot" {
            let mut node_pool = Vec::new();
            println!("digraph Trie {{");
            trie.dump_dot(&mut std::io::stdout(), &mut node_pool, 0);
            println!("}}");
        }else if args[1] == "complete" {
            let completions = trie.complete(args[2].clone());
            for completion in completions {
                println!("{}", completion);
            }
        }
        else {
            usage();
        }
    }
    else {
        usage();
    }
}
