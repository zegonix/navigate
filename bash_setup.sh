#!/usr/bin/env bash

export PATH="$PATH:$PWD/target/debug/"
pid=( $(ps -o ppid) )
arg_pid=" --pid ${pid[-2]} "


push() {
    \builtin cd -- "$(navigate ${arg_pid} push $@)"
}

pop() {
    \builtin cd -- "$(navigate ${arg_pid} pop $@)"
}

stack() {
	echo "$(navigate ${arg_pid} show $*)"
}
