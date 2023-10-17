# ![](assets/favicon.ico)  Mini RPS
Mini reverse proxy server written in rust

## Features
 - [X] very fast single binary with no dependencies
 - [X] static files server
 - [X] reverse proxy router
 - [X] https
 - [X] CORS
 - [X] consume any API data and create customized responses with minijinja templates
 - [X] extensively tested with hurl
 - [ ] cache rules
 - [ ] hot reload server in case of file changes
 - [ ] define once an array of requests based on a variable

## Usage
```
./target/release/minirps start config.toml
```

### Simple static file server
Config.toml
```toml
assets = "path/to/static/folder"
```

### Running on port 4000 instead of 3000
Config.toml
```toml
assets = "path/to/static/folder"
port = 4000
```

### Use https instead of http
Config.toml
```toml
assets = "path/to/static/folder"
port = 4000
cert = "path/to/cert.pem"
key = "path/to/key.pem"
```

### Allow cors from my site
Config.toml
```toml
assets = "path/to/static/folder"
port = 4000
cert = "path/to/cert.pem"
key = "path/to/key.pem"
cors = [
  "https://www.my-site.com"
]
```

### Allow cors from my sites in many scenarios
Config.toml
```toml
assets = "path/to/static/folder"
port = 4000
cert = "path/to/cert.pem"
key = "path/to/key.pem"
cors = [
  "http://www.my-site.com",
  "https://www.my-site.com",
  "http://www.my-other-site.com",
  "https://www.my-other-site.com"
]
```

### Allow cors from all origins
Config.toml
```toml
assets = "path/to/static/folder"
port = 4000
cert = "path/to/cert.pem"
key = "path/to/key.pem"
cors = []
```

### Add a reverse proxy to a API server running at http://localhost:8000 
Config.toml
```toml
assets = "path/to/static/folder"
port = 4000
cert = "path/to/cert.pem"
key = "path/to/key.pem"
cors = []

# GET https://localhost:4000/api/users => GET http://localhost:8000/users
[[routes]]
method = "GET"
path = "/api/users"

[[routes.requests]]
method = "GET"
url = "http://localhost:8000/users"

# PUT https://localhost:4000/api/users/21 => PUT http://localhost:8000/users/21
[[routes]]
method = "PUT"
path = "/api/users/:id"

[[routes.requests]]
method = "PUT"
url = "http://localhost:8000/users/{{params.id}}"
body = "{{body}}"
```

### Send text message response instead of API response
Config.toml
```toml
assets = "path/to/static/folder"
port = 4000
cert = "path/to/cert.pem"
key = "path/to/key.pem"
cors = []

# GET https://localhost:4000/api/users => GET http://localhost:8000/users
[[routes]]
method = "GET"
path = "/api/users"

[[routes.requests]]
name = "users"
method = "GET"
url = "http://localhost:8000/users"

[routes.response]
body = """
{% for user in data.users.json %}
  {{user.name}}
{% endfor %}"""
headers = { Content-Type = "text/plain" }

# PUT https://localhost:4000/api/users/21 => PUT http://localhost:8000/users/21
[[routes]]
method = "PUT"
path = "/api/users/:id"

[[routes.requests]]
name = "result"
method = "PUT"
url = "http://localhost:8000/users/{{params.id}}"
body = "{{body}}"

[routes.response]
body = "{% if data.result.status == 200 %}SUCCESS!{% else %}ERROR!{% endif %}"
headers = { Content-Type = "text/plain" }
```

### Send great HTML template response instead of API response
Config.toml
```toml
templates = "path/to/my/great/minijinja/templates/folder"
assets = "path/to/static/folder"
port = 4000
cert = "path/to/cert.pem"
key = "path/to/key.pem"
cors = []

# GET https://localhost:4000/api/users => GET http://localhost:8000/users
[[routes]]
method = "GET"
path = "/api/users"

[[routes.requests]]
name = "users"
method = "GET"
url = "http://localhost:8000/users"

[routes.response]
body = "{% include 'users.html' %}"
headers = { Content-Type = "text/html" }

# PUT https://localhost:4000/api/users/21 => PUT http://localhost:8000/users/21
[[routes]]
method = "PUT"
path = "/api/users/:id"

[[routes.requests]]
name = "result"
method = "PUT"
url = "http://localhost:8000/users/{{params.id}}"
body = "{{body}}"

[routes.response]
body = "{% include 'edit_users.html' %}"
headers = { Content-Type = "text/html" }
```

### Consume an open API star wars data 
```
./target/release/minirps new config.toml
./target/release/minirps start config.toml
```

## Docs
### config.toml
Currently any changes in `config.toml` you must reload the server to apply.

#### port: integer?
Optional port integer to run the server, default: 3000

#### cors: array of strings?
Optional array of allowed origins for CORS requests.
An empty array allow all origins.
If this variable is not setted CORS is disabled.

#### cert: string?
Optional string with the path of the public key file to https server.
Only if `cert` and `key` is available the server will run in https.

#### key: string?
Optional string with the path of the private key file to https server.
Only if `cert` and `key` is available the server will run in https.

#### assets: string?
Optional string with the path for the static site folder.
Currently any file added you must reload the server to apply.
The data inside the file is dynamically readed so files editions is already
covered.

#### templates: string?
Optional string with the path for your minijinja templates folder.
This folder is dinamically readed so you can edit files without problems.
But the templates inside `config.toml` are readed only on startup, and in case
of changes you must restart the server.

### Available template variables

## Tests

### http
Used to test a complete server without certs, it contains a static folder,
templates and dynamic routes

```
cp tests/http.toml config.toml
./target/release/minirps start config.toml
```

Running tests
```
hurl --test tests/http.hurl
```
