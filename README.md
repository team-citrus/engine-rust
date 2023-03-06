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
  - [ ] Create Rust component system compatible with the C++ system **<ins>(Highest priority)</ins>**
    - [ ] Optimize to minimize latency
    - [ ] Optimize to minimize code complexity
- [ ] Port **physics** API
- [ ] Port **graphics** API
- [ ] Replacements for standard library objects

### Medium priority

- [ ] Port `simd.h` and `simd.hpp` from C++
