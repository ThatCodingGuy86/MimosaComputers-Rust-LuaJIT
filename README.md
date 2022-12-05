## MimosaComputers Rust LuaJIT

This repository is a sister repository to [MimosaComputers](https://github.com/ThatCodingGuy86/MimosaComputers), as it is the source for the DLL required to run the mod.

The DLL is written in Rust and exports an API for the mod to interact with, and create, LuaJIT VMs.

## Install Instructions
* First, install [Rust](https://www.rust-lang.org/tools/install).
* Then, clone this repository either via Git or via pressing the "code"
  button in the upper right of the GitHub page,
  then pressing "download as ZIP", then extracting it to a drive with at least 600 MB of space.
* Then, run the command `cargo build` in the directory with the file `Cargo.toml` in it.  
This should produce a folder labeled `target`. Go into that folder, then go into
  the `debug` folder within that. There should be a file labeled `mimosa_computers_luajit.dll`.
* Copy that into your that into the top level of the folder your Minecraft install is in. (This is one directory above your `mods` folder.) 