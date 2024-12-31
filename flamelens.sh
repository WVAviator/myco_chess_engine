#!/bin/bash

cargo flamegraph --post-process 'flamelens --echo' --root --release -- --threads 1 --perft 6
