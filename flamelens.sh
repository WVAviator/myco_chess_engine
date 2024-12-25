#!/bin/bash

cargo flamegraph --post-process 'flamelens --echo' --root --release
