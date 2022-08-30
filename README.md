# rs-redis-queue

Provides a small example of using redis to build out a rust-based worker queue.

## Run Me
This relies on redis to provide the queue source of truth, so you can run a 
docker container to get that running in a terminal run:

```shell
docker run --name redis --rm -p 6376:6376 redis:7-alpine
```

To start the manager, in another terminal, run:

```shell
cargo run --manifest-path ./manager/Cargo.toml
```

To start the worker, in another/as many terminals as you want, run:
```shell
cargo run --manifest-path ./worker/Cargo.toml
```
