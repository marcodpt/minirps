# Checking assets folder

GET http://localhost:3000/
HTTP/1.1 200
Content-Type: text/html
[Captures]
home: body
[Asserts]
body contains "<title>MiniRPS Demo</title>"
body == {{home}}

GET http://localhost:3000/index.html
HTTP/1.1 200
Content-Type: text/html
[Asserts]
body == {{home}}

GET http://localhost:3000/favicon.ico
HTTP/1.1 200
Content-Type: image/x-icon
Content-Length: 15086
[Asserts]
bytes count == 15086

GET http://localhost:3000/certs/cert.txt
HTTP/1.1 200
Content-Type: text/plain
```
-----BEGIN CERTIFICATE-----
MIIDkzCCAnugAwIBAgIUXVYkRCrM/ge03DVymDtXCuybp7gwDQYJKoZIhvcNAQEL
BQAwWTELMAkGA1UEBhMCVVMxEzARBgNVBAgMClNvbWUtU3RhdGUxITAfBgNVBAoM
GEludGVybmV0IFdpZGdpdHMgUHR5IEx0ZDESMBAGA1UEAwwJbG9jYWxob3N0MB4X
DTIxMDczMTE0MjIxMloXDTIyMDczMTE0MjIxMlowWTELMAkGA1UEBhMCVVMxEzAR
BgNVBAgMClNvbWUtU3RhdGUxITAfBgNVBAoMGEludGVybmV0IFdpZGdpdHMgUHR5
IEx0ZDESMBAGA1UEAwwJbG9jYWxob3N0MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8A
MIIBCgKCAQEA02V5ZjmqLB/VQwTarrz/35qsa83L+DbAoa0001+jVmmC+G9Nufi0
daroFWj/Uicv2fZWETU8JoZKUrX4BK9og5cg5rln/CtBRWCUYIwRgY9R/CdBGPn4
kp+XkSJaCw74ZIyLy/Zfux6h8ES1m9YRnBza+s7U+ImRBRf4MRPtXQ3/mqJxAZYq
dOnKnvssRyD2qutgVTAxwMUvJWIivRhRYDj7WOpS4CEEeQxP1iH1/T5P7FdtTGdT
bVBABCA8JhL96uFGPpOYHcM/7R5EIA3yZ5FNg931QzoDITjtXGtQ6y9/l/IYkWm6
J67RWcN0IoTsZhz0WNU4gAeslVtJLofn8QIDAQABo1MwUTAdBgNVHQ4EFgQUzFnK
NfS4LAYuKeWwHbzooER0yZ0wHwYDVR0jBBgwFoAUzFnKNfS4LAYuKeWwHbzooER0
yZ0wDwYDVR0TAQH/BAUwAwEB/zANBgkqhkiG9w0BAQsFAAOCAQEAk4O+e9jia59W
ZwetN4GU7OWcYhmOgSizRSs6u7mTfp62LDMt96WKU3THksOnZ44HnqWQxsSfdFVU
XJD12tjvVU8Z4FWzQajcHeemUYiDze8EAh6TnxnUcOrU8IcwiKGxCWRY/908jnWg
+MMscfMCMYTRdeTPqD8fGzAlUCtmyzH6KLE3s4Oo/r5+NR+Uvrwpdvb7xe0MwwO9
Q/zR4N8ep/HwHVEObcaBofE1ssZLksX7ZgCP9wMgXRWpNAtC5EWxMbxYjBfWFH24
fDJlBMiGJWg8HHcxK7wQhFh+fuyNzE+xEWPsI9VL1zDftd9x8/QsOagyEOnY8Vxr
AopvZ09uEQ==
-----END CERTIFICATE-----
```

GET http://localhost:3000/certs/key.txt
HTTP/1.1 200
Content-Type: text/plain
```
-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDTZXlmOaosH9VD
BNquvP/fmqxrzcv4NsChrTTTX6NWaYL4b025+LR1qugVaP9SJy/Z9lYRNTwmhkpS
tfgEr2iDlyDmuWf8K0FFYJRgjBGBj1H8J0EY+fiSn5eRIloLDvhkjIvL9l+7HqHw
RLWb1hGcHNr6ztT4iZEFF/gxE+1dDf+aonEBlip06cqe+yxHIPaq62BVMDHAxS8l
YiK9GFFgOPtY6lLgIQR5DE/WIfX9Pk/sV21MZ1NtUEAEIDwmEv3q4UY+k5gdwz/t
HkQgDfJnkU2D3fVDOgMhOO1ca1DrL3+X8hiRabonrtFZw3QihOxmHPRY1TiAB6yV
W0kuh+fxAgMBAAECggEADltu8k1qTFLhJgsXWxTFAAe+PBgfCT2WuaRM2So+qqjB
12Of0MieYPt5hbK63HaC3nfHgqWt7yPhulpXfOH45C8IcgMXl93MMg0MJr58leMI
+2ojFrIrerHSFm5R1TxwDEwrVm/mMowzDWFtQCc6zPJ8wNn5RuP48HKfTZ3/2fjw
zEjSwPO2wFMfo1EJNTjlI303lFbdFBs67NaX6puh30M7Tn+gznHKyO5a7F57wkIt
fkgnEy/sgMedQlwX7bRpUoD6f0fZzV8Qz4cHFywtYErczZJh3VGitJoO/VCIDdty
RPXOAqVDd7EpP1UUehZlKVWZ0OZMEfRgKbRCel5abQKBgQDwgwrIQ5+BiZv6a0VT
ETeXB+hRbvBinRykNo/RvLc3j1enRh9/zO/ShadZIXgOAiM1Jnr5Gp8KkNGca6K1
myhtad7xYPODYzNXXp6T1OPgZxHZLIYzVUj6ypXeV64Te5ZiDaJ1D49czsq+PqsQ
XRcgBJSNpFtDFiXWpjXWfx8PxwKBgQDhAnLY5Sl2eeQo+ud0MvjwftB/mN2qCzJY
5AlQpRI4ThWxJgGPuHTR29zVa5iWNYuA5LWrC1y/wx+t5HKUwq+5kxvs+npYpDJD
ZX/w0Glc6s0Jc/mFySkbw9B2LePedL7lRF5OiAyC6D106Sc9V2jlL4IflmOzt4CD
ZTNbLtC6hwKBgHfIzBXxl/9sCcMuqdg1Ovp9dbcZCaATn7ApfHd5BccmHQGyav27
k7XF2xMJGEHhzqcqAxUNrSgV+E9vTBomrHvRvrd5Ec7eGTPqbBA0d0nMC5eeFTh7
wV0miH20LX6Gjt9G6yJiHYSbeV5G1+vOcTYBEft5X/qJjU7aePXbWh0BAoGBAJlV
5tgCCuhvFloK6fHYzqZtdT6O+PfpW20SMXrgkvMF22h2YvgDFrDwqKRUB47NfHzg
3yBpxNH1ccA5/w97QO8w3gX3h6qicpJVOAPusu6cIBACFZfjRv1hyszOZwvw+Soa
Fj5kHkqTY1YpkREPYS9V2dIW1Wjic1SXgZDw7VM/AoGAP/cZ3ZHTSCDTFlItqy5C
rIy2AiY0WJsx+K0qcvtosPOOwtnGjWHb1gdaVdfX/IRkSsX4PAOdnsyidNC5/l/m
y8oa+5WEeGFclWFhr4dnTA766o8HrM2UjIgWWYBF2VKdptGnHxFeJWFUmeQC/xeW
w37pCS7ykL+7gp7V0WShYsw=
-----END PRIVATE KEY-----
```

GET http://localhost:3000/tests/data.json
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

# Checking blank route

GET http://localhost:3000/blank
```
Some body data
```
HTTP/1.1 200
[Asserts]
body == ""

POST http://localhost:3000/blank
```
Some body data
```
HTTP/1.1 405
[Asserts]
body == ""

PUT http://localhost:3000/blank
```
Some body data
```
HTTP/1.1 405
[Asserts]
body == ""

DELETE http://localhost:3000/blank
```
Some body data
```
HTTP/1.1 405
[Asserts]
body == ""

GET http://localhost:3000/blank/34
```
Some body data
```
HTTP/1.1 405
[Asserts]
body == ""

POST http://localhost:3000/blank/34
```
Some body data
```
HTTP/1.1 405
[Asserts]
body == ""

PUT http://localhost:3000/blank/34
```
Some body data
```
HTTP/1.1 200
[Asserts]
body == ""

DELETE http://localhost:3000/blank/34
```
Some body data
```
HTTP/1.1 405
[Asserts]
body == ""

# Extracting data showcase
POST http://localhost:3000/vars/345
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
POST http://localhost:3000/vars/test?x=60
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
GET http://localhost:3000/set/POST/pet?x=dog
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

GET http://localhost:3000/set/PUT/70
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
