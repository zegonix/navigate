#!/usr/bin/env bash

export PATH="$PATH:$PWD/target/debug/"
pid=( $(ps -o ppid) )
arg_pid=" --pid ${pid[-2]} "

nav() {
    cd "$(navigate ${arg_pid} $*)"
}

push() {
    cd "$(navigate push ${arg_pid} $*)"
}

pop() {
    cd "$(navigate pop ${arg_pid} $*)"
}

stack() {
    "navigate stack ${arg_pid}"
}