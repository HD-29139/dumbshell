
<p align="center">
  <img src="https://github.com/HD-29139/dumbshell/blob/main/assets/dshellv1.png" width="1024" alt="HD-29139 - DUMBSHELL">
</p>

# 🐒DUMBSHELL🐒

This is a minimalist shell written in **Rust**. It allows you to execute commands interactively, similar to a terminal.

- Reads user input line by line
- Splits the input into command and arguments
- Executes commands with arguments using `std::process::Command`
- Waits for each command to finish before continuing

## How It Works

The shell reads each line typed by the user, splits the input using whitespace, and uses the first word as the command. All remaining words are passed as arguments.

### Example

If you type:

```nushell
echo Hello, world!
```
The shell will execute:
```rust
Command::new("echo").args(["Hello,", "world!"])
```
And the output will be:
```
Hello, world!
```
### Try yourself🐒

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

### Just do this:
```nushell
git clone https://github.com/HD-29139/dumbshell.git
cd dumbshell
cargo run
```

