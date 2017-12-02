#!/bin/bash

function test {
    make run_$1 | grep $2 > /dev/null && echo -e "$1: \e[32mOK\e[39m" || echo -e "$1: \e[31mFAIL\e[39m"
}

test day_1_1 1177
test day_1_2 1060
