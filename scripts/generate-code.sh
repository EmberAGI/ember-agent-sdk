#!/bin/sh

mkdir temp-source && cd temp-source
openapi-generator-cli generate -g rust -i ../openapi-spec.yaml && \
    rm -rf ../src ../docs &&
    mv README.md src docs ../ && \
    cd .. && \
    rm -r temp-source