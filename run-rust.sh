#!/bin/bash

cargo build --release
time ./target/release/index
