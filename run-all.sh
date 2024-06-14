#!/bin/sh -x

FEATURES="${1:-default}"
MACHINE_TAG="${2:-unknown}"
VM="${3:-unknown}"
SEGMENT_SIZE="${4:-unknown}"

export RUSTFLAGS='-C target-cpu=native'

cargo run -F ${FEATURES} --release -- measure --segment-size=${SEGMENT_SIZE} ${MACHINE_TAG} ${VM} hello-world


bash -x run-sha2.sh ${FEATURES} ${MACHINE_TAG} ${VM} ${SEGMENT_SIZE}

bash -x run-fib.sh ${FEATURES} ${MACHINE_TAG} ${VM} ${SEGMENT_SIZE}

bash -x run-sort.sh ${FEATURES} ${MACHINE_TAG} ${VM} ${SEGMENT_SIZE}


bash -x run-big-input.sh ${FEATURES} ${MACHINE_TAG} ${VM} ${SEGMENT_SIZE}

bash -x run-big-input-push.sh ${FEATURES} ${MACHINE_TAG} ${VM} ${SEGMENT_SIZE}

bash -x run-big-input-vecless.sh ${FEATURES} ${MACHINE_TAG} ${VM} ${SEGMENT_SIZE}
