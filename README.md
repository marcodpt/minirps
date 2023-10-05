# ![](assets/favicon.ico)  Mini RPS
Mini reverse proxy server written in rust

## Usage
```
./target/release/minirps new config.toml
./target/release/minirps start config.toml
```

## TODO
 - implement redirect server
 - add minijinja to redirect server
 - parse body response json

 - add cache rules
 - add tests
 - check translations and write docs
 - hot reload server in case of file changes
 - publish first version
