## DCPU-16

DCPU-16 is a fictional, 16-bit virtual CPU spec. It was created in 2012 by Markus Pearson (Notch), the creator of Minecraft!
He was working on a space trading/simulator game and wanted to use this CPU spec in spaceships so people can
program it writing assembly and all. Although unfortunately the development of this game discontinued, I kinda wanted to play it ngl.
But as spec is available so I am making the emu.

### Plan

- [x] make a dcpu-16 emulator emulator
- [ ] make it cycle accurate
- [ ] add all the necessary hardware
- [x] make a assembler to convert assembly of dcpu into the 16 bit hex instructions
- [ ] advance assembly

```diff
- WIP
```

### Install and build

```bash
git clone https://github.com/aliqyan-21/dcpu-emu
cd dcpu-emu
cargo build
```

### Usage

```bash
./dcpu-emu --help
```

### License
[MIT](license.md)

### References
[dcpu spec](https://gist.github.com/metaphox/3888117)
