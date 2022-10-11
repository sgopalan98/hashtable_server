rm Results/*
rm Plots/*
cargo run --release -- -t $1
./scripts/plot_all_results.sh
