#!/usr/bin/env bash

export PATH="$PATH:$PWD/target/debug/"
pid=( $(ps -o ppid) )

nav() {
    cd "$(navigate ${pid[-2]} $*)"
}

push() {
    cd "$(navigate ${pid[-2]} push $*)"
}

pop() {
    cd "$(navigate ${pid[-2]} pop $*)"
}
