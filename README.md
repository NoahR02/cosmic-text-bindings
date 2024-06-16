```shell
cargo build
```

```shell
cbindgen --config cbindgen.toml --crate cosmic-text-c --output cosmic-text.h
```

```shell
mv target/debug/cosmic_text_c.lib lib/cosmic_text_c.lib
mv cosmic-text.h include/cosmic-text.h
```