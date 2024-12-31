#!/bin/bash

cargo flamegraph --post-process 'flamelens --echo' --root --release --freq 5000 -- --threads 1 --perft 6
