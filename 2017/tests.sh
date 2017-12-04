#!/bin/bash

function file_test {
    OUTPUT=$(make build/$1)
    if [ $? -ne 0 ]; then
        echo -e "$OUTPUT"
        exit 1
    fi

    OUTPUT=$(build/$1 < input/$2)
    if  [[ $OUTPUT == "$3" ]]; then
        printf "%s(%s) == %s: \e[32m%s\e[39m\n" "$1" "$2" "$3" "OK"
    else
        printf "%s(%s) == %s: \e[31m%s\e[39m\n" "$1" "$2" "$3" "FAIL"
        echo -e "$OUTPUT"
        exit 1
    fi
}

function data_test {
    OUTPUT=$(make build/$1)
    if [ $? -ne 0 ]; then
        echo -e "$OUTPUT"
        exit 1
    fi

    OUTPUT=$(echo -en "$2" | build/$1)
    if  [[ $OUTPUT == "$3" ]]; then
        printf "%s(%s) == %s: \e[32m%s\e[39m\n" "$1" "$2" "$3" "OK"
    else
        printf "%s(%s) == %s: \e[31m%s\e[39m\n" "$1" "$2" "$3" "FAIL"
        echo -e "$OUTPUT"
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
