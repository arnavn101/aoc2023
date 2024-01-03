#!/bin/bash
N=$1
PREV_N=${2:-$((N-1))}

echo "Creating files for day $N from day $PREV_N"

touch inputs/d${N}.txt
touch inputs/d${N}_sample.txt

cp -u src/d0.rs src/d${N}.rs
sed -i "s/d$PREV_N/d$N/g" src/main.rs