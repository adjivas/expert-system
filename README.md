# Expert-System

[![docs-badge][]][docs] [![license-badge][]][license] [![travis-badge][]][travis]

#### How to build:
```shell
git clone https://github.com/adjivas/expert-system expert_sys && cd expert_sys
cargo build
```

#### Rules:
* An axiom can imply another axiom and has a boolean value.
* An axiom is a axiom and a expression.
* A expression can imply another expression.
* A expression can be the result of another expressions.

#### Directory-Tree:
```shell
.
|__ Cargo.toml
|__ LICENSE
|__ README.md
|__ examples
│   |__ and.rs
│   |__ or.rs
│   |__ xor.rs
│   |__ not.rs
│   |__ solver.rs
│   \__ axiom.rs
|__ src
│   |__ lib.rs
│   |__ main.rs
│   |__ axiom.rs
│   |__ set.rs
│   |__ exp.rs
│   \__ ops
│       |__ and.rs
│       |__ or.rs
│       |__ xor.rs
│       |__ not.rs
│       \__ mod.rs
\__ tests
    |__ lib.rs
    |__ and.rs
    |__ or.rs
    |__ xor.rs
    |__ not.rs
    \__ axiom.rs
```

#### License:
*expert-system*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license][license].

[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: http://adjivas.github.io/expert-system/expert_sys
[license-badge]: http://img.shields.io/badge/license-GPLv3-blue.svg?style=flat-square
[license]: https://github.com/adjivas/expert-system/blob/master/LICENSE
[travis-badge]: https://travis-ci.org/adjivas/expert-system.svg?style=flat-square
[travis]: https://travis-ci.org/adjivas/expert-system
[circle-badge]: https://circleci.com/gh/adjivas/expert-system/tree/master.svg?style=svg
