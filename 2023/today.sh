#!/bin/sh

TODAY="$(date +%d)"
FILENAME="src/day$TODAY.rs"
cp template.rs "$FILENAME"
sed -i "s/DAYNUM/$TODAY/g" "$FILENAME"
cat template.toml | sed "s/DAYNUM/$TODAY/g" >> Cargo.toml
echo "Enter input:"
cat > "input/$TODAY"
cargo watch -x "run --bin day$TODAY"
