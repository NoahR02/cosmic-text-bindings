# cosmic-text-cpp (WIP)

![images/intro.png](images/intro.png)

## Build Instructions
### Build the system library
```shell
cargo build
```

### Move the system library to the lib folder
```shell
mv target/debug/cosmic_text_c.lib lib/cosmic_text_c.lib
```

### Generate the C++ header

```shell
cbindgen --config cbindgen.toml --crate cosmic-text-c --output include/cosmic-text.hpp
```

## User Instructions
Look in the examples folder for how to use this in your C++ project.