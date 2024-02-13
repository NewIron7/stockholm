# 🛠️ Harmless Ransomware Tool 🛠️
This tool is a harmless ransomware simulation designed strictly for educational purposes. It simulates the encryption and decryption process of files within a specified directory, showcasing the fundamental workings of ransomware. **Please use this tool responsibly and only on systems where you have explicit permission to do so.**
## 📝 Description
The harmless ransomware encrypts files in the `infection` folder located in the current user's home directory. It utilizes a key stored in the `.encrypt.key` file for encryption. If the `.encrypt.key` does not exist, the tool will generate a new key and store it in this file.
## 🚀 Getting Started
To use this tool, you'll need Rust installed on your system. You can compile and run the tool using Cargo, Rust's package manager and build system.
### Prerequisites
- Install Rust: Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install Rust and Cargo.
### Compilation
Compile the program by running `cargo build --release`. This command creates an executable in the `target/release` directory.
### Running the Tool
- To **encrypt files**, simply run the compiled program without any arguments. This will encrypt files in the specified `infection` and generate or use an existing `.encrypt.key` for the encryption key.
  ```shell
  ./target/release/stockholm
  ```
- To **decrypt files**, use the `--reverse` argument with the encryption key. If you don't specify a key, it will attempt to use the key found in `.encrypt.key`.
  ```shell
  ./target/release/stockholm --reverse=<YourEncryptionKey>
  ```
- Enable **silent mode** to suppress output by using the `--silent` flag. This is useful for running the program without terminal output.
  ```shell
  ./target/release/stockholm --silent
  ```
## 🖇️ Quick test
You can start a quick test of stockholm by running:
```shell
make
```
This will build the program in a container, so no need to have rust on your local machine, just `docker`.
During the container's creation, some test will be placed in the 'infection' folder.
Once built, run:
```shell
make test
```
and you are ready to go 🛫
The binary has been put in the `/usr/local/bin` folder so you can try `stockholm` just by running: 
```shell
stockholm -h
```
