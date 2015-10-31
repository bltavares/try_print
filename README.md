# try_print and try_println
## Non-panicking printing to stdout

This create provides an alternative print and println macro that don't panic.

The `println` and `print` macros provides a simple interface to output
content on the stdout of a program. The macros panic when writing to
stdout, and the failure condition could happen on external conditions.

Take the following rust code:

```rust
fn main() {
  for _ in 0..10000 {
    println!("line") {
  }
}
```

Piping the program output to other utilities could cause the program to
panic, when the pipe is closed.

```bash
produce_logs | head
line
line
line
line
line
line
line
line
line
line
thread '<main>' panicked at 'failed printing to stdout: Broken pipe (os error 32)', ../src/libstd/io/stdio.rs:588
```

Instead of panicking, it would be interesting to allow the developer to
decide what to do with the error result, either ignoring it or panicking
on it's own. This crate provides `try_println` and `try_print` as an
alternative non-panicking macros.

The following code will not panic anymore when the pipe is closed.

```rust
#[macro_use] extern crate try_print;

fn main() {
  for _ in 0..10000 {
    if let Err(_) = try_println!("line") {
      std::process::exit(0);
    }
  }
}
```
