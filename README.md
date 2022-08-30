# rs-redis-queue

Provides a small example of using redis to build out a rust-based worker queue.

## Run Me
These apps rely on redis to provide the queuing "source of truth", so you will 
need to have redis running. If you have docker installed, pull and run a redis
container by running the following in a terminal:

```shell
docker run --name redis --rm -p 6379:6379 redis:7-alpine
```

To start the manager, in another terminal, run:

```shell
$ cargo run --manifest-path ./manager/Cargo.toml
cargo run --color=always --package manager --bin manager
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/manager`
Manager running!
sending job: 0
sending job: 1
sending job: 2
sending job: 3
...
```

To start the worker, in another/as many terminals as you want, run:
```shell
$ cargo run --manifest-path ./worker/Cargo.toml
cargo run --color=always --package worker --bin worker
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/worker`
Worker running!
no. of jobs in list = 580
marking job(0) as 'done done'
marking job(1) as 'done done'
marking job(2) as 'done done'
marking job(3) as 'done done'
```
