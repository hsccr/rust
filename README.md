# rust

Rocket Mounts
```
🛰  Mounting /:
    => GET /hello/<name>/<age> (hello)
    => GET /hello/<name> (hi)
    => HEAD / (httpd_head)
    => DELETE / (httpd_delete)
    => GET / (httpd_get)
    => GET /custom (httpd_get_custom)
    => GET /str (httpd_get_str)
    => GET /string (httpd_get_string)
    => GET /json (httpd_get_json)
    => PUT / (httpd_put)
    => POST / (httpd_post)
    => PATCH / (httpd_patch)
    => OPTIONS / (httpd_options)
```

HTTP GET
```
curl -X GET http://localhost:8000/hello/ccr -v
```

HTTP POST
```
curl -X POST -H "Content-Type: application/json" -d '{"name":"abc", "message":"message"}' http://localhost:8000 -v
```

