# bfrs
Brainfuck interpreter written in Rust, just for fun.

## Usage
```bash
> cargo build
   Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs

> ./target/debug/bfrs $(cat hello.bf)
Hello World!
----------------
exit code: 10
cell dump: [0, 0, 72, 100, 87, 33, 10]
```

## License
[MIT License](MIT)
