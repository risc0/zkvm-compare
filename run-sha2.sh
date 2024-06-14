#!/bin/sh -x

FEATURES="${1:-default}"
MACHINE_TAG="${2:-unknown}"
VM="${3:-unknown}"
SEGMENT_SIZE="${4:-unknown}"

SIZES=(
    1024
    4096
    8192
    16384
    32768
)

for SIZE in "${SIZES[@]}"
do
    cargo run -F ${FEATURES} --release -- measure --segment-size=${SEGMENT_SIZE} ${MACHINE_TAG} ${VM} sha2 ${SIZE}
done
