#!/bin/sh

TODAY="$(date +%d)"
FILENAME="src/day$TODAY.rs"
[ -f "$FILENAME" ] && exit 0
cp template.rs "$FILENAME"
sed -i "s/DAYNUM/$TODAY/g" "$FILENAME"
cat template.toml | sed "s/DAYNUM/$TODAY/g" >> Cargo.toml
echo "Enter input:"
cat >> "inputs/$TODAY.txt"
cargo watch -x "run --bin day$TODAY"
