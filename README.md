


RUST_LOG=TRACE ./target/debug/dpdkrs-demo --conf config.ini  --proc-type=primary --proc-id=0

RUST_LOG=TRACE ./target/debug/client --conf config.ini  --proc-type=primary --proc-id=0
