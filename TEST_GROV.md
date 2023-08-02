# Test grov - What is grov?

- This is a project initiated by Mozilla to gather code coverage results on Firefox

- test tutorial grov [from here](https://github.com/mozilla/grcov#how-to-get-grcov)

## steps

1. install grcov

    ```bash
    cargo install grcov
    ```

2. make project clean

   ```bash
   cargo clean
   ```

3. export variable into shell

   ```bash
   export CARGO_INCREMENTAL=0
   export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
   export RUSTDOCFLAGS="-Cpanic=abort"
   ```

4. build project

   ```bash
   cargo build
   ```

5. run test in folder examples src files

   ```bash
   cargo test --examples
   ```

6. create code coverage report in html

   ```bash
   grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
   ```

7. show report inside browser

   ```bash
   export DISPLAY=:0
   firefox /home/trapapa/test_rust_regex_file/target/debug/coverage/index.html
   ```
