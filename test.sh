#!/bin/bash
DISPLAY_INFO=$(./target/release/mindisplayinfo-rs)
DISPLAY_INFO_ARRAY=()
for i in $DISPLAY_INFO; do DISPLAY_INFO_ARRAY+=($i) ; done
echo "width: ${DISPLAY_INFO_ARRAY[0]} "
echo "height: ${DISPLAY_INFO_ARRAY[1]} "
echo "refresh: ${DISPLAY_INFO_ARRAY[2]} "
echo "ratio: ${DISPLAY_INFO_ARRAY[3]} "