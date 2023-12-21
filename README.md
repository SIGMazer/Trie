# Prefix Tree (Trie) Implementation in Rust

## Quick Start

### Build

```console
$ rustc trie.rs
```

### Dump the Trie as dot

```console
$ ./trie dot

```
### Dump the trie as svg
```console
$ dot -Tsvg trie.dot -o trie.dot.svg
$ xdg-open trie.dot.svg

```

### Autocomplete prefix

```console
$ ./trie complete Ap

```
