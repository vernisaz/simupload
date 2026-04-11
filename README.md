# Simple uploader

## Goal
Demonstrate using Rust to handle HTTP upload/download requests

## Dependencies
- [SimWeb](https://github.com/vernisaz/simweb)
 

## Status
The project is used in conjunction with [Simple Commander](https://github.com/vernisaz/simcom).

## Test
A test case assumes the entry in `env.conf` of [SimHTTP](https://github.com/vernisaz/simhttp) as :
```json
       {"path":"/cmd/upload", "_comment_": "Upload for Simple Commander",
       "CGI": true,
       "translated": "./../simupload"}
```

A test URL looks like : http://hostname/cmd/upload/html/index.html