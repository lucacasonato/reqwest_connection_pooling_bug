This demonstrates an issue with connection pooling in reqwest on HTTP/2 that
results in following error:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: reqwest::Error { kind: Request, url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("deno-website2.now.sh")), port: None, path: "/", query: None, fragment: None }, source: hyper::Error(Http2, Error { kind: Proto(NO_ERROR) }) }', src/main.rs:23:14
```

This repo contains two examples of the same code. One written in Go using the
net/http std lib, and one using reqwest w/ rustls on a single threaded tokio
event loop. Both examples spawn 100 concurrent tasks, each doing 20 requests to
https://deno-website2.now.sh/ in series.

To run the working Go example:

```
$ cd go
$ GOMAXPROCS=1 go run main.go
```

To run the reqwest example:

```
$ cd reqwest
$ cargo run --features pool
```

If instead you create a single client per connection (no connection pooling) you
do not experience this issue:

```
$ cd reqwest
$ cargo run
```
