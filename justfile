set shell := ["cmd.exe", "/c"]

create day:
    cargo generate --path ./daily-template --name day{{day}}

lint day:
    cargo clippy -p day{{day}}

run day part:
    cargo run -p day{{day}} --bin day{{day}}_part{{part}}

run_release day part:
    cargo run -p day{{day}} --bin day{{day}}_part{{part}} --release

test day part:
    cargo nextest run -p day{{day}} day{{day}}_part{{part}}

bench day part:
    cargo bench --bench day{{day}}-bench day{{day}}_part{{part}}
    
bench_alloc day part:
    cargo bench --bench day{{day}}-bench-alloc day{{day}}_part{{part}}
    
bench-all:
    cargo bench -q > benchmarks/benchmarks.txt

flamegraph day part:
    cargo flamegraph --profile flamegraph --root --package day{{day}} --bin day{{day}}_part{{part}} -o flamegraphs/day{{day}}--day{{day}}_part{{part}}.svg

dhat day part:
    cargo run --profile dhat --features dhat-heap --package day{{day}} --bin day{{day}}_part{{part}}

# work-run day part:
#     cargo watch --watch target/ -x "check -p day{{day}}"  -s "just lint day{{day}}" -x "run -p day{{day}} --bin day{{day}}_part{{part}}"

# # Use `just work day01 part1` to work on the specific binary for a specific day's problems
# work day part:
#     cargo watch --watch target/ -x "check -p day{{day}}" -s "just test day{{day}}_part{{part}} -p day{{day}}" -s "just lint day{{day}}" -s "just bench day{{day}} day{{day}}_part{{part}}"

# www-watch:
#     RUST_LOG=info cargo +nightly leptos watch --project www

# www-build:
#     cargo +nightly leptos build --project www --release

