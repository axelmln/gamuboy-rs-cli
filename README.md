# gamuboy-rs-cli
A GameBoy emulator running in the terminal

It uses [gamuboy-rs](https://github.com/axelmln/gamuboy-rs) as the emulation core.

## Usage

Build it from source:  
```
$ cd gamuboy-rs-cli
$ cargo build --release
```  

```
$ ./gamuboy /path/to/rom [--bootrom /path/to/bootrom]
```  


## Key bindings

| Game Boy        | Keyboard        |
|-----------------|-----------------|
| A               | `A`             |
| B               | `Z`             |
| Start           | `Enter`         |
| Select          | `Tab`           |
| D-Pad Up        | `Arrow Up`      |
| D-Pad Down      | `Arrow Down`    |
| D-Pad Left      | `Arrow Left`    |
| D-Pad Right     | `Arrow Right`   |
