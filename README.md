# ![](assets/favicon.ico)  Mini RPS
Mini [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) server written in rust

## Features
 - [X] very fast single binary with no dependencies
 - [X] static file server
 - [X] [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) router
 - [X] https
 - [X] [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
 - [X] consume any API data and create custom responses with [minijinja](https://github.com/mitsuhiko/minijinja) templates
 - [X] extensively tested with [hurl](https://github.com/Orange-OpenSource/hurl)
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

### Using https instead of http
Config.toml
```toml
assets = "path/to/static/folder"
port = 4000
cert = "path/to/cert.pem"
key = "path/to/key.pem"
```

### Allow [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) for my website
Config.toml
```toml
assets = "path/to/static/folder"
port = 4000
cert = "path/to/cert.pem"
key = "path/to/key.pem"
cors = [
  "https://www.my-website.com"
]
```

### Allow [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) from my websites of varying origins 
Config.toml
```toml
assets = "path/to/static/folder"
port = 4000
cert = "path/to/cert.pem"
key = "path/to/key.pem"
cors = [
  "http://www.my-website.com",
  "https://www.my-website.com",
  "http://www.my-other-website.com",
  "https://www.my-other-website.com"
]
```

### Allow [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) from all origins
Config.toml
```toml
assets = "path/to/static/folder"
port = 4000
cert = "path/to/cert.pem"
key = "path/to/key.pem"
cors = []
```

### Add a [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) to an API server running at http://localhost:8000 
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

### Send a plain text response instead of the API response
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

### Send HTML template response instead of API response
Config.toml
```toml
templates = "path/to/templates/folder"
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
body = "{% include 'edit.html' %}"
headers = { Content-Type = "text/html" }
```

## Docs
### config.toml
Currently, any changes to `config.toml`, the server must be restarted for them to be applied.

#### port: integer?
Optional integer port number to run the server on, default: 3000

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
Optional string with the static site folder path.

#### templates: string?
Optional string with the path to the [minijinja](https://github.com/mitsuhiko/minijinja) templates folder.

#### routes: [{method: string, path: string, requests: [{...}]?, response: {...}?}]
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
 - `path` is a string with the path associated with the route, `:var` is acceptable for setting path variables (ex: /api/user/:id).

#### routes.requests: [{name: string?, method: string, headers: {header: string}?, url: string, body: string?}]
Requests is an optional array of objects representing requests that needs to be done to generate response.
 - `name` is an optional string that will be used to (if present) to store the response data associated with the request to be available in [minijinja](https://github.com/mitsuhiko/minijinja) templates.
 - `method` is a required http method as described in the `routes` definition, but it is also a [minijinja](https://github.com/mitsuhiko/minijinja) template, so you are able to use some logic here.
 - `headers` is an object with the keys been the header to be setted in the request and the values a string containing the value of the header or a [minijinja](https://github.com/mitsuhiko/minijinja) template to generate it.
 - `url` is a required [minijinja](https://github.com/mitsuhiko/minijinja) template or a raw string associated with the request.
 - `body` is an optional [minijinja](https://github.com/mitsuhiko/minijinja) template or a raw string associated with the request.

#### routes.response {status: string?, headers: {header: string}?, body: string?}
Response starts with the `status`, `headers` and `body` of the response of the last request in the array of `requests` or if is not present an empty 200 response, and the properties here are modifiers to the response sended by the [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) to the client.
 - `status` is an optional string or a [minijinja](https://github.com/mitsuhiko/minijinja) template representing an integer to modify the status code of the response.
 - `headers` is an optional object where the keys are the headers to be setted or modified in the response and the values a string or a [minijinja](https://github.com/mitsuhiko/minijinja) template representing the value associated with the header.
 - `body` is an optional string or [minijinja](https://github.com/mitsuhiko/minijinja) template with the body to be replaced the original response body.

### Available [minijinja](https://github.com/mitsuhiko/minijinja) template variables

#### path: string
The associated path as passed by the client in the request.

Ex.: `/api/user/:id` => `/api/user/25`.

#### query: string?
The associated query string or `none` as passed by the client in the request.

Ex.: `http://localhost:3000/api/users?name=john` => `name=john`

#### headers: {header: string}
The associated object of the headers as passed by the client in the request.
Observe that all headers keys are *lowercase*

Ex: Content-Type: text/plain => {"content-type": "text/plain"}

#### params: {param: string}
The associated object of the path params associated with the client request on a given route.

Ex: `/api/user/:id` => `http://localhost:3000/api/user/25` => {"id": "25"}

#### vars: {param: string}
The associated object of the query params associated with the client request.

Ex.: `http://localhost:3000/api/users?name=john` => `{"name": "john"}`

#### body: string
The body passed by the client in the request.

#### json
The body passed by the client in the request json parsed, if it fails it also
contains the body as a json string.

#### data: {name: {status: integer, headers: {header: string}, body: string, json}}
The data object is where is stored all results of the [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) array of requests. A result is stored only if there is a name associated with the request and is already available for the next request or response associated templates.
 - name: The keys of the object is the name passed in the array of `requests`.
 - status: The response status associated with the request.
 - headers: The response headers associated with the request (header name is always *lowercase* in this object)
 - body: The response body as string associated with the request.
 - json: Teh response body json parsed (or if fails as string) associated with the request.

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
