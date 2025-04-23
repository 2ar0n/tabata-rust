#!/bin/sh
cd tabata_rp2040
cp memory.x ..
cargo run --release tabata-rp2040
cd ..
rm memory.x