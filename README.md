# CPU6502.rs

A classic 6502 emulator written completely in Rust along with assembly programs I wrote for the CPU.

## Goal

The goal of this project was to concrete my understanding of how CPU emulators work while learning Rust. My previous experience from making [chip8.cpp](https://github.com/Muhammed-Rajab/chip8.cpp/) helped me tremendously, while the clarity of rust types made it quite enjoyable to work on.

I implemented pretty much all the instructions required to pass [Klaus Dormann's Functional Test](https://github.com/Klaus2m5/6502_65C02_functional_tests), and it works.

Cycle accuracy is to be implemented in the future, but right now I'm busy with another project.

I've planned to add a memory mapped display so I can write programs which can do some basic rendering (will be added soon).
