#!/bin/zsh

for pdb in ./PP/*.pdb
do
    ./target/release/pdb_split $pdb >> out.log
done