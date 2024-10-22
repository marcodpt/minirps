# ![](favicon.ico)  Mini RPS

[![Version](https://img.shields.io/crates/v/minirps)](https://crates.io/crates/minirps)
[![Downloads](https://img.shields.io/crates/d/minirps)](https://crates.io/crates/minirps)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/marcodpt/minirps/blob/main/LICENSE)

Mini [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) server
written in rust

## ❤️ Features
 - Static file server based on [axum](https://github.com/tokio-rs/axum).
 - HTTPS
 - [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
 - The optional configuration file can be written in
[JSON](https://www.json.org/json-en.html) or
[TOML](https://toml.io/en/).
 - [minijinja](https://github.com/mitsuhiko/minijinja) templates with custom
functions:
   - read, write and remove files from the filesystem. 
   - Send http requests in the template.
   - Execute commands in the template.
   - [Reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy). 
   - Modify the response headers and status in the template.
   - Parse and format to in the template to:
     - [JSON](https://www.json.org/json-en.html)
     - [TOML](https://toml.io/en/)
     - [FormData](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST)
 - Safe rust and good code organization.
 - No panics after startup (Every panic is a bug).
 - Extensively tested with [hurl](https://github.com/Orange-OpenSource/hurl).
 - Good debugging experience with the server displaying requests in the
terminal and error messages in templates for humans.
 - Designed following the principles of
[UNIX philosophy](https://en.wikipedia.org/wiki/Unix_philosophy).

## 💻 Install
```
cargo install minirps
```

Alternatively you can use one of the precompiled binaries available with each
release (currently generic Linux only).

## 🎮 Usage
### Help
```
minirps -h
```

### Simple static file server
```
minirps path/to/static/folder
```

### Serve hidden files
```
minirps -a path/to/static/folder
```

### Ignore markdown files in root folder
```
minirps -i "/*.md" path/to/static/folder
```

### Ignore any markdown files
```
minirps -i "/**/*.md" path/to/static/folder
```

### Running on port 4000 instead of 3000
```
minirps -p 4000 path/to/static/folder
```

### Using https instead of http
```
minirps -p 4000 path/to/static/folder -c path/to/cert.pem -k path/to/key.pem
```

### Allow [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) from all origins
```
minirps -o -p 4000 path/to/static/folder -c path/to/cert.pem -k path/to/key.pem
```

### Start the server with a config file
The supported formats are JSON and TOML.
```
minirps -f path/to/config/file
```

### Send HTML template response instead of API response
Here it is assumed that there are
[minijinja](https://github.com/mitsuhiko/minijinja) templates `users.html`
and `edit_user.html`

config.toml
```toml
templates = "path/to/templates/folder"
assets = "path/to/static/folder"
port = 4000
cert = "path/to/cert.pem"
key = "path/to/key.pem"
cors = []

[[routes]]
method = "GET"
path = "/api/users"
template = "users.html"

[[routes]]
method = "GET"
path = "/api/users/:id"
template = "edit_user.html"

[[routes]]
method = "POST"
path = "/api/users/:id"
template = "edit_user.html"
```

Alternatively you can use a JSON file

config.json
```json
{
  "templates": "path/to/templates/folder",
  "assets": "path/to/static/folder",
  "port": 4000,
  "cert": "path/to/cert.pem",
  "key": "path/to/key.pem",
  "cors": [],
  "routes": [
    {
      "method": "GET",
      "path": "/api/users",
      "template": "users.html"
    }, {
      "method": "GET",
      "path": "/api/users/:id",
      "template": "edit_user.html"
    }, {
      "method": "POST",
      "path": "/api/users/:id",
      "template": "edit_user.html"
    }
  ]
}
```

## 💯 Examples

### Demo
```
minirps -f examples/demo/config.toml
minirps -f examples/demo/config.json
```

Here it was implemented:
 - Command Line: use of the command line through a
[minijinja](https://github.com/mitsuhiko/minijinja) custom function.
 - Periodic Table: A periodic table web interface was built from a JSON file.
 - Star Wars API: Web interface for [swapi](https://swapi.dev/) Star Wars API.
 - Note taking app: An example using the file system to save and read data.
 - Form Data: Sending and reading examples.
 - CORS: A working demo of a CORS request, needs both servers running. 

### test
In this example, a static server and some routes are built to test the use of
reverse proxy and templates automatically using
[hurl](https://github.com/Orange-OpenSource/hurl).

```
minirps -f examples/tests/config.toml
```

```
hurl --test examples/tests/test.hurl
```

## 📢 Motivation

## 📖 Docs
### config
Command line arguments take priority over config file if both are present.  

Command line argument paths are relative to the current working directory.

`config` paths are relative to your own directory.

Currently, any changes to `config.toml`, the server must be restarted for them to be applied.

#### port: integer?
Optional integer port number to run the server on, default: 3000

#### all: bool
Whether to display hidden files. 

In case of confirmation via the command line or `config.toml` they will be
displayed.

#### ignore: [string]?
List of files to ignore using glob expressions.

If the -i option is passed on the command line it will be appended to the list.

The routes must be considered in relation to the assets folder and not the
working directory.

For a complete reference of glob expressions and possible bugs check this
[library](https://github.com/devongovett/glob-match).

#### cors: [string]?
Optional array of strings representing allowed origins for [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) requests.

An empty array allows all origins.

If this variable is not defined,[CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) will be disabled.

#### cert: string?
Optional string with the public key file path for the https server.

Only if the `cert` and `key` are available will the server run over https.

#### key: string?
Optional string with the private key file path for the https server.

Only if the `cert` and `key` are available will the server run over https.

#### assets: string?
Optional string with the static files folder path.

#### templates: string?
Optional string with the path to the
[minijinja](https://github.com/mitsuhiko/minijinja) templates folder.

#### data: string?
Optional string with the path where templates can read, write and remove files.
If not passed, these functions will be unavailable to templates.

#### routes: [{method: string, path: string, template: string}]
Optional array of objects that define [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) routes:
 - `method` is a string with one of the http methods:
   - GET
   - POST
   - DELETE
   - PUT
   - PATCH
   - HEAD
   - OPTIONS
   - TRACE
   - CONNECT
 - `path` is a string with the path associated with the route, `:var` is
acceptable for setting path variables (ex: /api/user/:id).
 - `template` is the template path associated with this route within the
`templates` folder.

### Template variables

#### method: string
The `method` associated with this `route`. It is useful when the same template
is used in many `routes`.

#### url: string
It is the junction of the `path` and the `route` `query`.

Ex.: `http://localhost:3000/api/users?name=john#me` => `/api/users?name=john`

#### route: string
It is the `route` as declared in the `config` file.

Ex.: `/api/user/:id`.

#### path: string
The associated `path` passed by the client in the request.

Ex.: `/api/user/:id` => `/api/user/25`.

#### query: string?
The associated `query` string passed by the client in the request.

Ex.: `http://localhost:3000/api/users?name=john` => `name=john`

#### params: {param: string}
The associated object of the `path` `params` associated with the client
request on a given `route`.

Ex: `/api/user/:id` => `http://localhost:3000/api/user/25` => {"id": "25"}

#### vars: {param: string}
The associated object of the `query` params associated with the client request.

Ex.: `http://localhost:3000/api/users?name=john` => `{"name": "john"}`

#### headers: {header: string}
The associated object of the headers passed by the client in the request.

Note that all header keys are in **lowercase**.

Ex: Content-Type: text/plain => {"content-type": "text/plain"}

#### body: binary
The body passed by the client in the request.

### Modify response status and headers within the template
An example of a redirect.

```jinja
{% set modify = {"status": 303, "headers": {"Location": "/new/location"}} %}
```
 - status (integer?): The new response status code, if not passed, will use
200 by default.
 - headers ({name: value}?): The headers that should be changed in the
response.

### Reverse proxy within the template
An example of a reverse proxy.

```jinja
{% set proxy = {"url": "https://another_random_host_ip"~url} %}
```
 - url (string): The proxy URL, is required.
 - method (string?): The method used for the proxy request. By default, the
method passed in the original request.
 - headers ({name: value}?): The headers that should be changed in the
proxy request. By default, do not change any header.
 - body (binary?): The body of the proxy request. By default,
the original body.

### Custom functions

#### command (cmd: string) -> {code: integer, stdout: binary, stdin: binary}

#### read

#### write

#### remove

#### get

#### delete

#### head

#### options

#### post

#### put

#### patch

### Custom filters

#### parse (data: binary, encoding: string) -> any

#### format (data: any, encoding: string) -> string

#### bytes (data: string) -> binary

## 📦 Releases
Currently, only binaries for generic versions of Linux are distributed across
releases.
```
sudo apt install pkg-config libssl-dev musl-tools
rustup update
rustup target add x86_64-unknown-linux-musl
cargo update
cargo build --release --target x86_64-unknown-linux-musl
```

## 🤝 Contributing
It's a very simple project.
Any contribution, any feedback is greatly appreciated.

## ⭐ Support
If this project was useful to you, consider giving it a star on github, it's a
way to increase evidence and attract more contributors.

## 🙏 Acknowledgment
This work would not be possible if it were not for these related projects:
 - [minijinja](https://github.com/mitsuhiko/minijinja)
 - [axum](https://github.com/tokio-rs/axum)
 - [reqwest](https://github.com/seanmonstar/reqwest)
 - [hurl](https://github.com/Orange-OpenSource/hurl)
 - [serde](https://github.com/serde-rs/serde)
 - [clap](https://github.com/clap-rs/clap)
 - [glob-match](https://github.com/devongovett/glob-match)

A huge thank you to all the people who contributed to these projects.
