#!/bin/bash

cp target/debug/makewiz test-dirs/test-makefile-make
cd test-dirs/test-makefile-make

> log.txt

./makewiz | tee -a log.txt
make | tee -a log.txt
./main | tee -a log.txt
