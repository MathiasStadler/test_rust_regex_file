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

// The output is wrapped cat <<EOT >> ./examples/first_read_line.rsin a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
EOT
```

## [How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

- Add this to first_read_line.rs

```bash
cat <<EOT >> ./examples/first_read_line.rs

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
EOT
```

## Now run all test in subfolder examples

```bash
cargo test --examples
```
> output

```shell
trapapa@trapapa-ThinkPad-T430:~/test_rust_regex_file$ cargo test --examples
   Compiling test_rust_regex_file v0.1.0 (/home/trapapa/test_rust_regex_file)
    Finished test [unoptimized + debuginfo] target(s) in 0.34s
     Running unittests examples/first_read_line.rs (target/debug/examples/first_read_line-991179279f279ee9)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests examples/hello_examples.rs (target/debug/examples/hello_examples-37289ea276810f8a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## test grov [from here](https://github.com/mozilla/grcov#how-to-get-grcov)

1. make project clean

    ```bash
    cargo clean
    ```
2. export variable into shell

    ```bash
    export CARGO_INCREMENTAL=0
    export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
    export RUSTDOCFLAGS="-Cpanic=abort"
    ```

3. build project

    ```bash
    cargo build
    ```

4. run test in folder examples src files

    ```bash
    cargo test --examples
    ```

5. create code coverage report in html

    ```bash
    grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
    ```

## delete all empty line in the document

- HIER weiter
  - https://github.com/mozilla/rcov#how-to-get-grcov

- flamegraph
