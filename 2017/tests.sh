#!/bin/bash

function file_test {
    OUTPUT=$(make build/$1)
    if [ $? -ne 0 ]; then
        echo $OUTPUT
        exit 1
    fi
    OUTPUT=$(build/$1 < input/$2)
    if  echo $OUTPUT | grep "$3" > /dev/null; then
        echo -e "$1($2) == $3: \e[32mOK\e[39m"
    else
        echo -e "$1($2) == $3: \e[31mFAIL\e[39m"
        echo $OUTPUT
        exit 1
    fi
}

function data_test {
    OUTPUT=$(make build/$1)
    if [ $? -ne 0 ]; then
        echo $OUTPUT
        exit 1
    fi
    OUTPUT=$(echo -n "$2" | build/$1)
    if  echo $OUTPUT | grep "$3" > /dev/null; then
        echo -e "$1($2) == $3: \e[32mOK\e[39m"
    else
        echo -e "$1($2) == $3: \e[31mFAIL\e[39m"
        echo $OUTPUT
        exit 1
    fi
}

data_test day_1_1 1122 3
data_test day_1_1 1111 4
data_test day_1_1 1234 0
data_test day_1_1 91212129 9
file_test day_1_1 day_1.txt 1177

data_test day_1_2 1212 6
data_test day_1_2 1221 0
data_test day_1_2 123425 4
data_test day_1_2 123123 12
data_test day_1_2 12131415 4
file_test day_1_2 day_1.txt 1060
