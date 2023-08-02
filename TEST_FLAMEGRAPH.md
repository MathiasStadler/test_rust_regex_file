# test flamegraph benchmark

- [FROM HERE](https://github.com/killercup/cargo-flamegraph)

## steps

0.install flamegraph

```bash
cargo install flamegraph    
```

1.add config to Cargo.toml

[profile.release]
debug = true

2.Enabling perf for use by unprivileged users

```bash
:TODO make a script test and set
cat /proc/sys/kernel/perf_event_paranoid
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
```

3.perf need additional package for the kernel

```bash
sudo apt install linux-tools-5.19.0-50-generic
sudo apt install linux-tools-generic
```

4.cargo clean && cargo build

```bash
cargo clean && cargo build --release
```

5.run release version

```bash
cargo flamegraph --release
```