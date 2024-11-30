#!/usr/bin/env bash

export PATH="$PATH:$PWD/target/debug/"
pid=( $(ps -o ppid) )
arg_pid=" --pid ${pid[-2]} "

__call_navigate() {
    eval "$(navigate ${arg_pid} $@)"
}

push() {
    __call_navigate "push $@"
}

pop() {
    __call_navigate "pop $@"
}

stack() {
    __call_navigate "stack $@"
}

book() {
    __call_navigate "bookmark $@"
}
