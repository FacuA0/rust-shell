# Rust Shell

**Rust Shell** (still deciding on a new name) is a very simple command-line shell made in Rust. As of now it supports navigating through files (```cd``` and ```ls```), manipulating them (```md```, ```touch``` and ```rm```), and executing local files and those of the PATH variable.

## Commands

- ```cd```: Navigate between directories.
- ```ls```: Show the files and directories in the current location.
- ```md```: Make a directory.
- ```touch```: Create a file.
- ```rm```: Remove a file, a directory or a tree.
- ```help```: Shows the available commands.
- ```version```: Shows the current version.
- ```exit```: Exits the shell.

## Download and run

1. First, you need to have Rust version 1.74.1 installed. Go to [the official site](https://rust-lang.com/tools/install) for instructions.
2. If you have Git, open a terminal and paste:
   ```sh
   git clone https://github.com/FacuA0/rust-shell.git
   ```
   Then:
   ```bash
   cd rust-shell
   ```
   If you don't have Git, at the top of the page click on Code > Download Zip. Then extract the file and open a terminal inside of it.
3. Build and run it:
   ```bash
   cargo run
   ```