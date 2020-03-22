#!/bin/sh

t=$(cargo run --release Config.toml)
echo $t
python visualization.py output/$t.csv