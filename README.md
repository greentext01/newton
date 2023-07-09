# Newton

## Running

### Server

From the root directory of the project, execute the following:
```
cargo run --package server -- --system .\systems\default.ron
```

### Client

From the `flight` directory, execute the following:
```
cargo run
```

You must run it from the `flight` directory because assets are loaded with a path relative to the current directory.
