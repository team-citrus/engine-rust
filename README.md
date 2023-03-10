# The Rust Bindings for Citrus Engine

The Rust bindings for the [Citrus Engine](https://github.com/team-citrus/engine/).

Rust is a powerful language, and because of that, it is vital that we provide support for Rust in the Citrus Engine.
In providing these Rust bindings, we hope to bring in more use for the Citrus Engine, and to make it more useful and powerful.

## Compiling Requirements

- All requirements of the Citrus Engine
- rustc
- cargo

## TODOs

### High priority

- [ ] Port **core** API
  - [ ] Replacements for standard library objects
    - [x] `MemBox<T>`
    - [x] `Vector<T>`
      - [x] Add capacity
    - [x] `Pair<A, B>`
    - [ ] `Map<A, B>`
    - [ ] `HashMap<K, T>`
  - [x] Create Rust component system compatible with the C++ system **<ins>(Highest priority)</ins>**
    - [x] Optimize to minimize latency
    - [x] Optimize to minimize code complexity
    - [ ] Find some way to make Rust and C++ see into each other
- [ ] Port **physics** API
- [ ] Port **graphics** API

### Medium priority

- [ ] Port `simd.h` and `simd.hpp` from C++
