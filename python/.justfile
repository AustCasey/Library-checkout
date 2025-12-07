# https://just.systems/man/en/
# https://github.com/casey/just?tab=readme-ov-file
# usage: 
#   `just` to display a list of commands
#   `just run` to execute the `run` label
#   `just test` to execute the `test` label
#

# Show all available commands
default:
    @just --list --unsorted

# Run Python app
run:
    python3 -B -m app -q

# Run pytest for the app
test:
    python3 -m pytest -q


