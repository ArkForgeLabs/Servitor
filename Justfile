typeshare:
    typeshare ./server --lang=typescript --output-file=./editor/src/lib/types.ts

run-debug-server:
    cd server && cargo build && sudo target/debug/server