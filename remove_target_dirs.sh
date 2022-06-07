#!/bin/bash

for d in */; do
  cd "$d" 
  ls
  cd ..
done
