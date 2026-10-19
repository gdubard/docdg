#!/bin/sh
# Double-cliquable dans le Finder : compile docdg en mode release et
# assemble docdg.app. Ne dépend d'aucun fichier caché — un simple
# `cargo run --release -p xtask` suffit si vous préférez le terminal.
cd "$(dirname "$0")"
cargo run --release -p xtask
echo
echo "Terminé. Fermez cette fenêtre, ou glissez docdg.app dans /Applications."
read -r _ignore
