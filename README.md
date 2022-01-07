The issue has been solved. Thanks [@seanmonstar](https://github.com/seanmonstar)! <3

---

This demonstrates an issue with connection pooling in reqwest on HTTP/2 that
results in following error:

```
error sending request for url (https://deno-website2.now.sh/): http2 error: protocol error: not a result of an error
```

This repo contains two examples of the same code. One written in Go using the
net/http std lib, and one using reqwest w/ rustls on a single threaded tokio
event loop. Both examples start 100 concurrent goroutines/futures, each doing 20 requests to
https://deno-website2.now.sh/ in series.

To run the working Go example:

```
$ cd go
$ GOMAXPROCS=1 go run main.go
```

To run the reqwest example:

```
$ cd reqwest
$ cargo run
```

If instead you create a single client per connection (no connection pooling) you
do not experience this issue:

```
$ cd reqwest
$ cargo run --features no-pool
```
