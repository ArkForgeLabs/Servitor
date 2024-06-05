prepare-db docker-ps:
    docker -it exec {{docker-ps}} bash

build-node port container path type:
    cd services && docker build --build-arg NODE_SCRIPT={{path}} --build-arg TYPE={{type}} --build-arg PORT={{port}} -t {{container}} .

run-node port container path type:
    just build-node {{port}} {{container}} {{path}} {{type}}
    docker run -it -p {{port}}:{{port}} --cpus=1.0 -e cpuload=50 --memory="25m" --memory-swap="50m" {{container}}
    
start-db:
    docker compose up

stop-db:
    docker compose down

typeshare:
    typeshare ./server --lang=typescript --output-file=./editor/src/lib/types.ts

run-debug-server:
    cd server && cargo build && sudo target/debug/server