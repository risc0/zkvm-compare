#!/bin/sh -x

FEATURES="${1:-default}"
MACHINE_TAG="${2:-unknown}"
VM="${3:-unknown}"
SEGMENT_SIZE="${4:-unknown}"

SIZES=(
    131072
    524288
    2097152
    8388608
    33554432
)

for SIZE in "${SIZES[@]}"
do
    cargo run -F ${FEATURES} --release -- measure --segment-size=${SEGMENT_SIZE} ${MACHINE_TAG} ${VM} fib ${SIZE}
done
