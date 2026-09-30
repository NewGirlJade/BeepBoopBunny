#!/usr/bin/env bash
set -euo pipefail

elf_path="$1"
shift
gba_path="${elf_path}.gba"

agb-gbafix "$elf_path" -o "$gba_path"
# exec mgba-qt "-3" "$gba_path" "$@"
exec mGBA-0.10.5-appimage-x64.appimage "-3" "$gba_path" "$@"

