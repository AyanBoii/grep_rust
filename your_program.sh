#!/bin/sh

# Exit early if any commands fail
set -e

# Build the project
(
  cd "$(dirname "$0")" # Ensure compile steps are run within the repository directory
  cargo build --release
)

# Run the program with all arguments passed to the script
exec "$(dirname "$0")/target/release/your_program" "$@"