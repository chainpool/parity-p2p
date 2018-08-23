# parity-p2p

- Node1 run
``` terminal
RUST_LOG=debug ./target/debug/p2p
```

- Node2 run
``` terminal
RUST_LOG=debug ./p2p --bootnodes /ip4/127.0.0.1/tcp/20222/p2p/QmR7NQab1eyT73UtmVMekdbCMjgevCZFKcG4qAHTLuVdLR --port 33333
```
**/ip4/127.0.0.1/tcp/20222/p2p/QmR7NQab1eyT73UtmVMekdbCMjgevCZFKcG4qAHTLuVdLR** is Node1's URL
