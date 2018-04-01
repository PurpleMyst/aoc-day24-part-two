#!/usr/bin/env sh
set -e
set -x

me=$(whoami)

sudo apt install linux-tools-common linux-tools-generic "linux-tools-$(uname -r)"

cargo build --release

sudo perf record -F 99 -a -g -- target/release/aoc-day24-part-two

if [ ! -d FlameGraph ]; then
    git clone https://github.com/brendangregg/FlameGraph
fi

sudo chown "$me:$me" perf.data
mv perf.data FlameGraph
cd FlameGraph

perf script | ./stackcollapse-perf.pl > out.perf-folded
./flamegraph.pl out.perf-folded > perf-kernel.svg

$BROWSER perf-kernel.svg
