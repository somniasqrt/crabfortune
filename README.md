<div align="center">

# crabfortune 🦀🔮

**`fortune`, rewritten in Rust. Because why not.**

![written in Rust](https://img.shields.io/badge/written%20in-Rust-orange?logo=rust)
![speed](https://img.shields.io/badge/speed-1.8x%20faster-brightgreen)
![dependencies](https://img.shields.io/badge/dependencies-0-blue)
![wisdom](https://img.shields.io/badge/wisdom-questionable-yellow)

</div>

```
$ crabfortune --crab | crabsay
 ________________________
< panic!("at the disco") >
 ------------------------
        \
         \
           _~^~^~_
       \) /  o o  \ (/
         '_   ^   _'
         / '-----' \
```

## Why?

`fortune` was written in C in the 1970s. C is a fine language that has been fine for fifty
years. Unfortunately, it is not Rust.

This is part of the [crabsay](https://github.com/somniasqrt/crabsay) roadmap:

- [x] Rewrite `cowsay` in Rust
- [x] Rewrite `fortune` in Rust
- [ ] Rewrite everything else in Rust

## Benchmark

`hyperfine -N --warmup 20`:

| implementation | mean time |
|---|---|
| `fortune` (C) | 4.6 ms |
| **`crabfortune` (Rust)** | **2.5 ms** |

**~1.8× faster** than a C program from the 1970s. 🚀

> [!NOTE]
> The original does more work (locale handling, offensive-fortune logic, strfile indexes).
> On a huge fortune database it may well win. We have not tested that, on purpose.

## Install

```sh
cargo install --git https://github.com/somniasqrt/crabfortune
```

It reads your existing fortune databases, so install some:

```sh
brew install fortune
```

Without them, it still works: 23 built-in crab fortunes are compiled into the binary.

## Usage

```sh
crabfortune
crabfortune --crab
crabfortune -s
crabfortune -l
crabfortune -c
crabfortune ~/my-quotes.txt
crabfortune | crabsay
```

| flag | meaning |
|---|---|
| `--crab` | only built-in crab fortunes |
| `-s` | short fortunes only (≤ 160 chars) |
| `-l` | long fortunes only (> 160 chars) |
| `-n N` | change the short/long threshold |
| `-c` | show which file the fortune came from |
| `-e` | every file equally likely, not every fortune |
| `file\|dir` | use these instead of the default databases |

Fortune files are plain text, with entries separated by a line containing only `%`. No
`strfile` index needed: the crab reads the whole file. It's fast enough. It's Rust.

## Contributing

New crab fortunes go in `src/crab.txt`. They must be original, short, and at least slightly
about Rust or crabs.

## See also

- [crabsay](https://github.com/somniasqrt/crabsay): `cowsay`, rewritten in Rust. Gives your
  fortune a crab to say it:

  ```sh
  crabfortune | crabsay
  ```

## License

MIT © somniasequitur
