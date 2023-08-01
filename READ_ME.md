# MOTIVATION: Read file and remove empty lines with regex

## test example/hello_examples

```bash
cargo build
cargo run --example hello_examples
```

## read lines of file

> [from here](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html)

```rust
cat <<EOT >> ./examples/first_read_line.rs
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./READ_ME.md") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
EOT
```

## delete all empty line in the document
