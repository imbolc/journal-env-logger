[![License](https://img.shields.io/crates/l/journal-env-logger.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/journal-env-logger.svg)](https://crates.io/crates/journal-env-logger)
[![Docs.rs](https://docs.rs/journal-env-logger/badge.svg)](https://docs.rs/journal-env-logger)

<!-- cargo-sync-readme start -->

# journal-env-logger

Logging into system journal based on `RUST_LOG` environment variable

## Usage
```rust,no_run
// Initialize logging into journal
journal_env_logger::init_journal().unwrap();

// Initialize logging into stdout (e.g. for dev purposes)
journal_env_logger::init_stdout().unwrap();

// A helper to initialize stdout on dev and journal on prod
let is_prod = true;
journal_env_logger::init(is_prod).unwrap();
```

<!-- cargo-sync-readme end -->

## Contributing

We appreciate all kinds of contributions, thank you!


### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-sync-readme][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on commit.


## License

This project is licensed under the [MIT license](LICENSE).

[cargo-sync-readme]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
