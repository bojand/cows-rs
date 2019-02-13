# cows

> 400+ ASCII 🐮s

Get a list of cows or a random cow. The `Rust` version. Original [JavaScript Cows](https://github.com/sindresorhus/cows).

## Usage

**(Once published)** Add to your `Cargo.toml`:

```toml
[dependencies]
cows = "1.0"
```

```rust
extern crate cows;

fn main() {
    println!("{}", cows::random());
}
```

## CLI

```sh
$ cows
                   (oooo)
                 oo( -0 )oo
     / ------------  \/
    /  |          |
   /   |  ----\_/ |
> <    |||      |||
       |||      |||
        ~~       ~~
  Wendy Wellesley Cow-
```

## Examples

```
         (__)
         (oo)
  /-------\/
 / |     ||
+  ||----||
   ~~    ~~
     Cow


       \(:)/
       (o|o)
  /-----\_/
 /|      |
^ ||----||
  ^^    ^^
 Klingon Cow


                 ________________
         ^__^   /                \
         (oo)  ( Milk is logical. )
  /-------\/ --'\________________/
 / |     ||
*  ||W---||
   ^^    ^^
Mr Spock's cow
```


## Related

- [JavaScript Cows](https://github.com/sindresorhus/cows) - Original Javascript ASCII cows package
- [vaca](https://github.com/sindresorhus/vaca) - Get a random ASCII cow 🐮
- [cows-docker](https://github.com/alexellis/cows-docker) - ASCII cows on Docker
- [Swift Cows](https://github.com/NozeIO/Noze.io/tree/develop/Sources/cows) - ASCII cows in Swift
- [Elixir Cows](https://github.com/sotojuan/excows) - ASCII cows in Elixir
- [Haskell Cows](https://github.com/passy/cows-hs) - ASCII cows in Haskell
- [Go Cows](https://github.com/bojand/cows)

## License

MIT © [Bojan D](http://github.com/bojand)
