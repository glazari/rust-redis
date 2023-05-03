# Rust Redis

This is a rust replica of some of the functionalities that Redis has.
This is a project for me learning Rust, it is not meant to be used in production and I do not represent neither the Rust Project or Redis.


# How to use

Install the service

```bash
cargo install --path .
```

Startup a server

```bash
rust-redis server
```

In a separate terminal, startup the client repl

```bash
rust-redis client
```

There is also a standalone version has the datastore in the same process as the repl. Mostly for debuging.
```bash 
rust-redis repl
```

## Commands

```
set <key> <value>
get <key>
```


## Todo
- [ ] add a list keys command 
- [ ] add metrics (either statsd or cloudwatch?)
- [ ] add logging to file
- [ ] allow using `get` and `set` as keys or values
- [ ] add auto complete to repl
- [ ] loadtest application

