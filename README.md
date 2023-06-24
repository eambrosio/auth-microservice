# auth-microservice
Rust-based auth server, client and heath-check services

This project aims to show a simple case using gRPC and async calls. Simply for fun and learning, nothing fancy.

NOTE: this project is an exercise from the [Let's Get Rusty Bootcamp](https://letsgetrusty.com/bootcamp/)

## How to run
1. To run the server:
```bash
cargo watch -c -q -w src/auth-service -x "run -q --bin auth"
```
2. To run the health-check:
```bash
cargo watch -c -q -w src/health-check-service -x "run -q --bin health-check"
```
3. Regarding the client, you can ask for some help from your terminal:

```bash
./target/debug/client --help
```
and you should see something similar to this:
```bash
Usage: client [COMMAND]

Commands:
  sign-in   
  sign-up   
  sign-out  
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
and then, you just need to pass the arguments following the command instructions.

## TODO
- CI/CD integration, based on Github actions