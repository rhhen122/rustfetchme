# RustFetchMe

<a href="https://codeberg.org/rhhen122/rustfetchme">CodeBerg</a>
<a href="https://github.com/rhhen122/rustfetchme">GitHub</a>

<div align="center">
<img src="image.png" height="30">
</div>

Please have the latest version of `cargo` & `rustc` installed to your system

First use maybe slow, this is because your system has to allocate memory to the program.
it seems to be slower on first boot on `arm64` systems.

###
Install

```bash
git clone https://codeberg.org/rhhen122/rustfetchme.git
```

###
Update

```bash
git pull origin master
```

###
src/main.rs

```rust
// Linux
let mut os = "linux"; // Set a OS
let subsys = "ubuntu"; // Only for linux
let mut de = "KDE Plasma 9"; // A Desktop Enviroment
```
```rust
// MacOS
let mut os = "mac"; // Set a OS
let subsys = ""; // Only for linux
let mut de = ""; // A Desktop Enviroment
```

###
Compile

```bash
# Linux
cargo install --path .
```
```zsh
# MacOS ZSH
cat "/Users/YourUserHere/.cargo/bin" | cat ~/.zshrc -
cargo install --path .
```
```bash
# MacOS BASH
cat "/Users/YourUserHere/.cargo/bin" | cat ~/.bashrc -
cargo install --path .
```

###
Run with

```bash
rustfetchme
```