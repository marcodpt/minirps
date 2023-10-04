# ![](favicon.ico)  Mini RPS
Mini reverse proxy server written in rust

## Usage
```
./target/release/minirps new config.toml
./target/release/minirps start config.toml
```

## TODO
 - implement assets server
 - implement redirect server
 - add minijinja to redirect server
 - add https
 - add cors
 - parse body response json
 - add tests
 - hot reload server in case of file changes
 - add cache rules
 - check translations
 - publish first version
