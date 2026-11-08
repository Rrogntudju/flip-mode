#!/usr/bin/env bash
FILE="$HOME/.config/cosmic/com.system76.CosmicTheme.Mode/v1/is_dark"

if [ ! -f "$FILE" ]; then
    echo "$FILE n'existe pas"
    exit 1
fi

if [ "$(cat "$FILE")" = "true" ]; then
    echo "false" > "$FILE"
else
    echo "true" > "$FILE"
fi