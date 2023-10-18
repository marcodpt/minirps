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

#### routes.requests: [{name: string?, method: string, headers: {header: string}?, url: string, body: string?}]?
Requests is an optional array of objects that represent requests that need to be made to generate the response.
 - `name` is an optional string that will be used (if present) to store the response data associated with the request to be made available in [minijinja](https://github.com/mitsuhiko/minijinja) templates.
 - `method` is a required string containing the http method (or a [minijinja](https://github.com/mitsuhiko/minijinja) template) as described in the routes definition.
 - `headers` is an object with the keys been the header to be setted in the request and the values a string containing the value of the header or a [minijinja](https://github.com/mitsuhiko/minijinja) template to generate it.
 - `headers` is an object with the keys being the header to be configured in the request and the values being a string containing the header value or a [minijinja](https://github.com/mitsuhiko/minijinja) template to generate it.
 - `url` is a required [minijinja](https://github.com/mitsuhiko/minijinja) template or a raw string associated with the request.
 - `body` is an optional [minijinja](https://github.com/mitsuhiko/minijinja) template or a raw string associated with the request.

#### routes.response {status: string?, headers: {header: string}?, body: string?}?
The response starts with the status, headers, and response body of the last request in the requests array, or if not present an empty 200 response, and the properties here are modifiers of the response sent by the [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) to the client.
 - `status` is an optional string or [minijinja](https://github.com/mitsuhiko/minijinja) template that represents an integer to modify the status code of the response.
 - `headers` is an optional object where the keys are the headers to be modified in the response and the values are a string or a [minijinja](https://github.com/mitsuhiko/minijinja) template representing the value associated with the header.
 - `body` is an optional string or [minijinja](https://github.com/mitsuhiko/minijinja) template with the body to be replaced with the original response body.

### Available [minijinja](https://github.com/mitsuhiko/minijinja) template variables

#### path: string
The associated path passed by the client in the request.

Ex.: `/api/user/:id` => `/api/user/25`.

#### query: string?
The associated query string or `none` passed by the client in the request.

Ex.: `http://localhost:3000/api/users?name=john` => `name=john`

#### headers: {header: string}
The associated object of the headers passed by the client in the request.

Note that all header keys are in **lowercase**.

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
The body passed by the client in the request converted to json.

If it fails contains the body as a json string.

#### data: {name: {status: integer, headers: {header: string}, body: string, json}}
The data object is where all the results of the [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) request array are stored. A result is stored only if there is a name associated with it and will be available for the next request or response templates.

 - `name`: Object keys are the `name` passed in the `requests` array.
 - `status`: The response status associated with the request.
 - `headers`: The response headers associated with the request (the header name is always **lowercase** in this object)
 - `body`: The response body as a string associated with the request.
 - `json`: The response body converted to json (or if it fails as json string) associated with the request.

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
