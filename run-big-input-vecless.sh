#!/bin/sh -x

FEATURES="${1:-default}"
MACHINE_TAG="${2:-unknown}"
VM="${3:-unknown}"
SEGMENT_SIZE="${4:-unknown}"

SIZES=(
    256
    1024
    4096
    16384
    65536
    262144
    1048576
)

for SIZE in "${SIZES[@]}"
do
    cargo run -F ${FEATURES} --release -- measure --segment-size=${SEGMENT_SIZE} ${MACHINE_TAG} ${VM} big-input-vecless ${SIZE}
done
