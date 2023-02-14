#!/usr/bin/env bash
## Build release all code directories in the repostitory using cargo build --release.
#

for DIR in */; do
    DIRNAME=$(basename "$DIR")
    (echo "==> $DIRNAME <==")
    (cd $DIR && (cargo build --release || echo "Build failed."))
done

echo "Release complete."