# We-Git-You

Because git shouldn't be hard to use.

## Motivation
While git is a powerful tool it ends up having a lot of extra baggage associated
with it, arcane commands, and annoying to deal with man pages. Tools like git
are used by people, so they should be easy to use by people, while still
remaining flexible and powerful. `we-git-you` is a fresh look at what git could
be, including working with popular services like GitHub, Atlassian, and GitLab.

## Build requirements

You'll need `libgit2` installed on your system as well as having rustc and cargo
installed on your computer.

## How to build the binary

```bash
cargo build --release
```

This will give you a binary to use found under the `target` folder.

## Should I use it?

Probably not. This is real early stage stuff so your laundry and or code might get
eaten.

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
