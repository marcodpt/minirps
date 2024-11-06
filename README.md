# ![](favicon.ico)  Mini RPS

[![Version](https://img.shields.io/crates/v/minirps)](https://crates.io/crates/minirps)
[![Downloads](https://img.shields.io/crates/d/minirps)](https://crates.io/crates/minirps)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/marcodpt/minirps/blob/main/LICENSE)

Mini [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) server
written in rust

## ❤️ Features
 - Static file server.
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
   - Parse and format to:
     - [JSON](https://www.json.org/json-en.html)
     - [TOML](https://toml.io/en/)
     - [FormData](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST)
 - Safe rust and good code organization.
 - No panics after startup (every panic is a bug).
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
minirps path/to/static/folder -c path/to/cert.pem -k path/to/key.pem
```

### Allow [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) from all origins
```
minirps -o path/to/static/folder
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
```
alternatively
```
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

### Test
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
The objective of this project is to deliver an http server in a single
self-contained binary.

Where the basics should be obtained without any configuration file:
 - static file server.
 - HTTPS
 - CORS

And where other reverse proxy functionalities are obtained with simple
configurations.

Templates have the ability to send requests, read and write files and execute
commands.

This way they can interact with resources such as databases without the need
for a complete scripting language such as php, python, ruby...

A small, highly extensible server, without having to manage operating system
versions, dependencies and packages.

It simply works!

## 📖 Docs
### config
Command line arguments take priority over config file if both are present.  

Command line argument paths are relative to the current working directory.

`config` paths are relative to your own directory.

Currently, any changes to `config`, the server must be restarted for them
to be applied.

#### port: integer?
Optional integer port number to run the server on, default: 3000

#### all: bool
Whether to display hidden files. 

In case of confirmation via the command line or `config` file they will be
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
Optional string with the path where templates can `read`, `write` and `remove`
files. If not passed, these functions will be unavailable to templates.

#### routes: [{method, path, template}]
Optional array of objects that define routes:

 - `method` string: one of the http methods:
   - GET
   - POST
   - DELETE
   - PUT
   - PATCH
   - HEAD
   - OPTIONS
   - TRACE
   - CONNECT
 - `path` string: the path associated with the route, `:var` is
acceptable for setting path variables (ex: /api/user/:id).
 - `template` string: the template path associated with this route within the
`templates` folder.

### Template variables

#### method: string
The `method` associated with this `route`. It is useful when the same template
is used in many `routes`.

#### url: string
It is the junction of the `path` and the `route` `query`.

```
http://localhost:3000/api/users?name=john#me => /api/users?name=john
```

#### route: string
It is the `route` as declared in the `config` file.

```
/api/user/:id
```

#### path: string
The associated `path` passed by the client in the request.

```
http://localhost:3000/api/users?name=john => /api/users
```

#### query: string?
The associated `query` string passed by the client in the request.

```
http://localhost:3000/api/users?name=john => name=john
```

#### params: {name: value}
The associated object of the `path` `params` associated with the client
request on a given `route`.

 - `name` string: The name of the parameter as declared in the `route`.
 - `value` string: The value of the parameter passed in the `path`.

```
/api/user/:id => http://localhost:3000/api/user/25 => {"id": "25"}
```

#### vars: {name: value}
The associated object of the `query` params associated with the client request.

 - `name` string: The name of the parameter passed in the `query`.
 - `value` string: The value of the parameter passed in the `query`.

```
http://localhost:3000/api/users?name=john => {"name": "john"}
```

#### headers: {name: value}
The associated object of the headers passed by the client in the request.

Note that all header keys are in **lowercase**.

 - `name` string: The name of the header passed in the request.
 - `value` string: The value of the header passed in the request.

```
Content-Type: text/plain => {"content-type": "text/plain"}
```

#### body: binary
The body passed by the client in the request.

### Template return state
Variables that, if defined, modify the behavior of the server response.

It only works if they are **declared outside the blocks**
to be returned in the template's global state.

#### modify {status, headers: {name: value}}
The response body is always the result of the template, and this variable
allows you to modify the status code and headers.

 - `status` (integer?): The new response status code, if not passed, will use
200 by default.
 - `headers` ({name: value}?): The headers that should be changed in the
response.

An example of a redirect.
```jinja
{% set modify = {"status": 303, "headers": {"Location": "/new/location"}} %}
```

#### proxy {url, method, headers: {name, value}, body}
Uses a proxy instead of the template result.

 - `url` (string): The proxy URL, is required.
 - `method` (string?): The method used for the proxy request. By default, the
method passed in the original request.
 - `headers` ({name: value}?): The headers that should be changed in the
proxy request. By default, do not change any header.
 - `body` (binary?): The body of the proxy request. By default,
the original body.

A simple proxy that retains the request method, headers, body and path and just
directs it to another host.
```jinja
{% set proxy = {"url": "https://another.host.ip"~url} %}
```

### Custom functions

#### command (cmd) -> {code, stdout, stdin}
Executes a command passed in the template.

This function does not raise errors, in case of failure it returns the
`code` `999999`, and the error message.

 - `cmd` string: The command to be executed by the system.
 - `code` integer: The response code, in general zero indicates OK, and a
number greater than zero the error code.
 - `stdout` binary: The standard output of the executed command.
 - `stderr` binary: The error message returned.

List files in the current directory on UNIX systems.
```jinja
{% set res = command("ls -l") %}
{% set output = res.stdout | parse("text") %}
```

#### read (file) -> data
Reads the contents of a file, if it does not exist returns `None`.

This function does not raise errors, any read error will return `None`.

It will only be available if the `config` file contains the `data`
property with the folder that contains the files that can be read and modified.

 - `file` string: The path of the file to read.
 - `data` binary?: The contents of the file or `None` in case of errors.

```jinja
{% set content = read("some/file.json") | parse("json") %}
```

#### read (dir: string) -> [{...info}]
This function also works with a directory, which in this case will return an
array with information about the files contained in it.

 - `dir` string: If the path passed is a directory.

**info**
 - `accessed` string: Last access date (%Y-%m-%d %H:%M:%S).
 - `created` string: Creation date (%Y-%m-%d %H:%M:%S).
 - `modified` string: Modification date (%Y-%m-%d %H:%M:%S).
 - `is_dir` bool: True if it is a directory.
 - `is_file` bool: True if it is a file.
 - `is_symlink` bool: True if it is a symbolic link.
 - `name` string: Entry name.
 - `len` u64: Size in bytes.

```jinja
{% set content = read("some/dir") %}
{% for entry in content %}
  {{entry.name}}
{% endfor %}
```

#### write (file, data) -> error
Writes to a file. If necessary, create folders for the file. Always overwrites
content if it exists.

If an error occur, the error text will be returned, otherwise `None`.
Therefore, it does not raise errors.

It will only be available if the `config` file contains the `data`
property with the folder that contains the files that can be read and modified.

 - `file` string: The file path.
 - `data` binary: The raw data to be written.
 - `error` string?: Error message or `None`.

```jinja
{% set data = "Hello world!" %}
{{write("some/file.txt", data | bytes)}}
```

#### remove (entry) -> error
Removes a file or directory recursively.

If an error occur, the error text will be returned, otherwise `None`.
Therefore, it does not raise errors.

It will only be available if the `config` file contains the `data`
property with the folder that contains the files that can be read and modified.

 - `entry` string: The path of the file or directory to be removed.
 - `error` string?: Error message or `None`.

```jinja
{{remove("some/dir")}}
```

```jinja
{{remove("some/file.txt")}}
```

#### {method} (url, body) -> {status, headers, body}
Sends a synchronous request to an external resource.

This function does not raise errors, any error in the request will be returned
`status` code `400` with the `body` containing the error message.

 - `url` string: The URL of the request.
 - `body` binary: The body of the request.
 - `status` integer: The HTTP status code of the response.
 - `headers` {`name` string: `value` string}: Response headers.
 - `body` binary: Response body.
 - `method`:
   - `get` (url) -> {status, headers, body}
   - `delete` (url) -> {status, headers, body}
   - `head` (url) -> {status, headers, body}
   - `options` (url) -> {status, headers, body}
   - `post` (url, body) -> {status, headers, body}
   - `put` (url, body) -> {status, headers, body}
   - `patch` (url, body) -> {status, headers, body}

```jinja
{% set response = get("https://some/api") %}
{% set data = response.body | parse("json") %}
```

```jinja
{% set body = "some data" %}
{% set response = post("https://some/api", body | bytes) %}
{% set message = response.body | parse("text") %}
```

#### log (message) -> ()
Prints a message from the template on the terminal.

 - `message` string: The content of the message.

```jinja
{{ log("hi!") }}
```

### Custom filters

#### parse (data, encoding) -> result
Converts the raw data returned from some function to a template variable using
the passed encoding.

This function raises an `error` if you use an unsupported encoding or if the
decoding fails.

Returning the request with `status` code `500` in case of error.

 - `data` binary: Raw data returned from some function.
 - `encoding` string: The encoding to be used when reading the data.
Supported encodings:
   - form: [FormData](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST)  
   - json: [JSON](https://www.json.org/json-en.html)
   - toml: [TOML](https://toml.io/en/)
   - text: It just transforms the data into text.
 - `result`: A value supported by the template with associated data.

```jinja
{% set data = read("some/file.txt") | parse("text") %}
```

```jinja
{% set response = get("https://some/api") %}
{% set data = response.body | parse("json") %}
```

#### format (data, encoding) -> text
Converts a template variable to a formatted string.

This function raises an `error` if you use an unsupported encoding or if the
encoding fails.

Returning the request with `status` code `500` in case of error.

 - `data`: Any template variable.
 - `encoding` string: The type of encoding to be adopted when formatting the
text. Supported encodings:
   - form: [FormData](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST)  
   - json: [JSON](https://www.json.org/json-en.html)
   - toml: [TOML](https://toml.io/en/)
   - debug: Uses rust pretty print formatter.
 - `text` string: The text after encoding.

```jinja
{% set data = {"name": "John", "age": 30} %}
{% set text = data | format("form") %}
{{text}}
```

```
name=John&age=30
```

#### bytes (data) -> raw
Converts text to binary format.

 - `data` string: Any text.
 - `raw` binary: Text converted to binary.

```jinja
{% set error = write('hello.txt', 'Hello World!' | bytes) %}
```

```jinja
{% set response = post('http://myip/some/api', 'Hello World!' | bytes) %}
```

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
