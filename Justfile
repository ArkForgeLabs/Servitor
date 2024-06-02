prepare-db docker-ps:
    docker -it exec {{docker-ps}} bash
    
start-db:
    docker compose up

stop-db:
    docker compose down

typeshare:
    typeshare ./server --lang=typescript --output-file=./editor/src/lib/types.ts

run-debug-server:
    cd server && cargo build && sudo target/debug/server