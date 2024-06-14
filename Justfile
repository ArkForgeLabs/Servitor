prepare-db docker-ps:
    docker -it exec {{docker-ps}} bash
    
start-resources:
    docker compose up -d

stop-resources:
    docker compose down

typeshare:
    typeshare ./server --lang=typescript --output-file=./editor/src/lib/types.ts

build-editor:
    cd editor && bun run build

run-debug-server:
    cd server && cargo build && sudo target/debug/server

run-debug: build-editor run-debug-server
