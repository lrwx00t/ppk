# killport

Find the port, and kill it!

## Usage

### Crate Installation
```
❯ cargo install ppk

❯ ppk
Usage: ppk <port>

❯ sudo ppk 2222
```

### Run from source

```bash
# from source
❯ sudo cargo run -- 2222
Port 2222 is listening.
TCP 2222: [71064]
Found matching command: Process { pid: 71064, create_time: 1682278718.446401s, busy: 97.469151ms, instant: Instant { t: 3989048305352067 } }
Got 71064
Process 71064 terminated
```