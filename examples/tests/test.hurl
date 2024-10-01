# Checking assets folder

GET http://localhost:4000/
HTTP/1.1 200
Content-Type: text/html
[Captures]
home: body
[Asserts]
body contains "<title>MiniRPS Demo</title>"
body == {{home}}

GET http://localhost:4000/index.html
HTTP/1.1 200
Content-Type: text/html
[Asserts]
body == {{home}}

GET http://localhost:4000/favicon.ico
HTTP/1.1 200
Content-Type: image/x-icon
Content-Length: 15086
[Asserts]
bytes count == 15086

GET http://localhost:4000/tests/data.json
HTTP/1.1 200
Content-Type: application/json
```
{
  "x": 3,
  "y": "cat",
  "z": true,
  "method": "POST",
  "id": 38,
  "type": "text/plain"
}
```

GET http://localhost:4000/tests/deep/msg.txt
HTTP/1.1 200
Content-Type: text/plain
```
Deep message!
```

GET http://localhost:4000/.hidden.txt
HTTP/1.1 404
[Asserts]
body == ""

GET http://localhost:4000/.secret/hello.txt
HTTP/1.1 404
[Asserts]
body == ""

GET http://localhost:4000/tests/.hi.txt
HTTP/1.1 404
[Asserts]
body == ""

# Checking blank route

GET http://localhost:4000/blank
```
Some body data
```
HTTP/1.1 200
[Asserts]
body == ""

POST http://localhost:4000/blank
```
Some body data
```
HTTP/1.1 405
[Asserts]
body == ""

PUT http://localhost:4000/blank
```
Some body data
```
HTTP/1.1 405
[Asserts]
body == ""

DELETE http://localhost:4000/blank
```
Some body data
```
HTTP/1.1 405
[Asserts]
body == ""

GET http://localhost:4000/blank/34
```
Some body data
```
HTTP/1.1 405
[Asserts]
body == ""

POST http://localhost:4000/blank/34
```
Some body data
```
HTTP/1.1 405
[Asserts]
body == ""

PUT http://localhost:4000/blank/34
```
Some body data
```
HTTP/1.1 200
[Asserts]
body == ""

DELETE http://localhost:4000/blank/34
```
Some body data
```
HTTP/1.1 405
[Asserts]
body == ""

# Extracting data showcase
POST http://localhost:4000/vars/345
Content-Type: text/html
X-Set-Content: text/plain
`<h1>Data</h1>`
HTTP/1.1 200
Content-Type: text/plain
[Asserts]
body contains "path: /vars/345"
body contains "query: none"
body contains "headers['content-type']: text/html"
body contains "params.id: 345"
body contains "vars.x:"
body contains "body: <h1>Data</h1>"
body contains "json: <h1>Data</h1>"
body contains "status: 200"
body contains "crazy['content-type']: application/json"
body contains "json.id: 38"
body contains "\"z\": true"

# Extracting data showcase
POST http://localhost:4000/vars/test?x=60
Content-Type: text/html
X-Set-Content: text/plain
`<h1>Data</h1>`
HTTP/1.1 201
Content-Type: text/plain
[Asserts]
body contains "path: /vars/test"
body contains "query: x=60"
body contains "headers['content-type']: text/html"
body contains "params.id: test"
body contains "vars.x: 60"
body contains "body: <h1>Data</h1>"
body contains "json: <h1>Data</h1>"
body contains "status: 200"
body contains "crazy['content-type']: application/json"
body contains "json.id: 38"
body contains "\"z\": true"

# Dynamic route
GET http://localhost:4000/set/POST/pet?x=dog
`Some body data`
HTTP/1.1 200
Content-Type: application/json
[Asserts]
jsonpath "$.method" == "POST"
jsonpath "$.url" == "https://httpbin.org/anything/pet?x=dog"
jsonpath "$.args.x" == "dog"
jsonpath "$.data" == "Some body data"
jsonpath "$.headers.X-Set" == "some-data"
jsonpath "$.headers.X-Var" == "dog"

GET http://localhost:4000/set/PUT/70
`Some body data`
HTTP/1.1 200
Content-Type: application/json
[Asserts]
jsonpath "$.method" == "PUT"
jsonpath "$.url" == "https://httpbin.org/anything/70"
jsonpath "$.args" isEmpty
jsonpath "$.data" == "Some body data"
jsonpath "$.headers.X-Set" == "some-data"
jsonpath "$.headers.X-Var" == ""
