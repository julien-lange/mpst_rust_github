#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

# Compile protocols

## Ring
### Two
./scripts/create_files/compile.sh ring_two_binary 10 baking
./scripts/create_files/compile.sh ring_two_crossbeam 10 baking
./scripts/create_files/compile.sh ring_two_baking_cancel_inline 10 baking

### Three
./scripts/create_files/compile.sh ring_three_binary 10 baking
./scripts/create_files/compile.sh ring_three_crossbeam 10 baking
./scripts/create_files/compile.sh ring_three_baking_cancel_inline 10 baking

### Four
./scripts/create_files/compile.sh ring_four_binary 10 baking
./scripts/create_files/compile.sh ring_four_crossbeam 10 baking
./scripts/create_files/compile.sh ring_four_baking_cancel_inline 10 baking

### Five
./scripts/create_files/compile.sh ring_five_binary 10 baking
./scripts/create_files/compile.sh ring_five_crossbeam 10 baking
./scripts/create_files/compile.sh ring_five_baking_cancel_inline 10 baking

### Six
./scripts/create_files/compile.sh ring_six_binary 10 baking
./scripts/create_files/compile.sh ring_six_crossbeam 10 baking
./scripts/create_files/compile.sh ring_six_baking_cancel_inline 10 baking

### Seven
./scripts/create_files/compile.sh ring_seven_binary 10 baking
./scripts/create_files/compile.sh ring_seven_crossbeam 10 baking
./scripts/create_files/compile.sh ring_seven_baking_cancel_inline 10 baking

### Eight
./scripts/create_files/compile.sh ring_eight_binary 10 baking
./scripts/create_files/compile.sh ring_eight_crossbeam 10 baking
./scripts/create_files/compile.sh ring_eight_baking_cancel_inline 10 baking

### Nine
./scripts/create_files/compile.sh ring_nine_binary 10 baking
./scripts/create_files/compile.sh ring_nine_crossbeam 10 baking
./scripts/create_files/compile.sh ring_nine_baking_cancel_inline 10 baking

### Ten
./scripts/create_files/compile.sh ring_ten_binary 10 baking
./scripts/create_files/compile.sh ring_ten_crossbeam 10 baking
./scripts/create_files/compile.sh ring_ten_baking_cancel_inline 10 baking

### Eleven
./scripts/create_files/compile.sh ring_eleven_binary 10 baking
./scripts/create_files/compile.sh ring_eleven_crossbeam 10 baking
./scripts/create_files/compile.sh ring_eleven_baking_cancel_inline 10 baking

### Twenty
./scripts/create_files/compile.sh ring_twenty_binary 10 baking
./scripts/create_files/compile.sh ring_twenty_crossbeam 10 baking
./scripts/create_files/compile.sh ring_twenty_baking_cancel_inline 10 baking

# Run the benchmarks
cargo bench --bench ring --features="baking"  -- --verbose

## Concatenate all results in the results/ping_pong_mesh_ring.csv file
python3 scripts/create_graphs/ping_pong_mesh_ring.py