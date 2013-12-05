rustboot
--------
A small kernel written in Rust.

It paints the screen bright red and then displays some information. You can write. That's it:

![](http://i.imgur.com/XW8PUlM.png)

![](http://i.imgur.com/3cHXx2D.png)

## Setup

You need a few things to run rustboot:

1. [rust-core](https://github.com/thestinger/rust-core)
2. Rust's [`master` branch](https://github.com/mozilla/rust) or 0.9 release
3. qemu
4. On x86
  * clang
  * nasm
5. On ARM
  * binutils for arm-none-eabi
  * gcc cross-compiler
6. Optionally for debugging
  * gdb
  * tmux

Clone this repository and update rust-core.

```bash
$ git clone https://github.com/pczarn/rustboot.git
$ cd rustboot
$ git submodule update --init
### you can also pull latest rust-core:
$ git submodule foreach git pull origin master
```

To get edge Rust going, grab it from git:

```bash
$ git clone https://github.com/mozilla/rust
$ cd rust
$ ./configure
$ make && make install
```

### Arch Linux

Simply install all dependencies:
```
# pacman -S qemu nasm rust clang gdb tmux
# yaourt -S gcc-arm-none-eabi
```

### OSX

To set things up on OSX, do this:

Install `nasm` and `qemu` from homebrew:

```bash
$ brew install nasm
$ brew install quemu
```

Install binutils from source.

```bash
$ wget 'ftp://sourceware.org/pub/binutils/snapshots/binutils-2.23.52.tar.bz2'
$ ./configure --target=i386-elf
$ make && make install
```

## Running it

To compile, simply execute `make` command.

To run, use:
```bash
$ make run	# emulate default platform (x86)
$ make run arch=arm	# run on ARM
$ make debug arch=arm	# debug on ARM
```
