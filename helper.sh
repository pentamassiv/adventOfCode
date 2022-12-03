#!/bin/bash

set -e           # Immediatly exit if any of the commands return a non-zero exit status
set -o pipefail  # If any of multiple piped commands fails, the return code will be used for the whole pipe

explain_input() {
    echo "To download the input file, run:"
    echo "$0 input year day"
    echo "To submit an answer, run:"
    echo "$0 submit year day level answer"
    echo "It is important to enter the day WITHOUT leading zeros"
    echo ""
    echo "Examples"
    echo "Download the input file of day 1 of the 2022 AoC:"
    echo "$0 input 22 1"
    echo "Submit the answer "1000" for level 2 of day 10 of the 2022 AoC"
    echo "$0 submit 22 10 2 1000"
    exit 1
}

if [[ $# > 2 ]]; then
    mode="$1"
    year="$2"
    day="$3"
fi

if [ "$mode" == "input" ] && [[ $# == 3 ]]; then
    curl -b secret_session_cookie https://adventofcode.com/20$year/day/$day/input > 20$year/day$day/input
    exit 0
fi

if [ "$mode" == "submit" ] && [[ $# == 5 ]]; then
    level="$4"
    answer="$5"
    #echo "Would have sent:"
    echo "curl -X POST -b secret_session_cookie -H 'Content-Type: application/x-www-form-urlencoded' --data-raw 'level=$level&answer=$answer' https://adventofcode.com/20$year/day/$day/answer"
    curl -X POST -b secret_session_cookie -H 'Content-Type: application/x-www-form-urlencoded' --data-raw 'level=$level&answer=$answer' https://adventofcode.com/20$year/day/$day/answer
    exit 0
fi

explain_input
