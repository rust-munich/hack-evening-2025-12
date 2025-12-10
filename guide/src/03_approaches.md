# Approaces

## Data loading

The [input.txt](https://raw.githubusercontent.com/rust-munich/hack-evening-2025-12/refs/heads/main/input.txt) data is available for no AoC account holders.

- [std lib io api](https://doc.rust-lang.org/std/io/index.html) with `File::open` and `BufReader::new(file).lines()`
- `include_str!` macro to include the file as a `&'static str` ([docs](https://doc.rust-lang.org/std/macro.include_str.html))
- `include_bytes!` macro to include the file as a `&'static [u8]` ([docs](https://doc.rust-lang.org/std/macro.include_bytes.html))

## Grid representation

- `Vec<Vec<Cell>>` where `Cell` is an enum representing empty or paper roll or a simple `char`
- `HashSet<(usize, usize)>` storing coordinates of paper rolls only
- Flat `Vec<Cell>` with width and height stored separately for index calculations

## Neighbor calculation

- Hardcoded offsets for the 8 directions
- Using `itertools::iproduct!` macro to generate offsets
- Custom iterator that yields neighbor coordinates
- Optional: async iterator for neighbor calculation using `async-stream` crate

### Iterators

- Using standard iterator methods like `filter`, `map`, `flat_map`, and `collect`
- Implementing a custom iterator
```rust
// iterator trait:
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```
- Example: Fibonacci iterator
```rust
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        let curr = self.curr;
        self.curr = self.next;
        self.next = new_next;
        Some(curr)
    }
}
```
