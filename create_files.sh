#!/bin/bash
N=$1

touch inputs/d${N}.txt
touch inputs/d${N}_sample.txt

cp src/d0.rs src/d${N}.rs