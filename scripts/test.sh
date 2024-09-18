#!/bin/bash
cargo_source="/var/lib/jenkins/.cargo/env"
source $cargo_source

err_redirrect="/dev/null"

# Tests
cli_test="cli"
garden_controller_test="database::garden_controller"
container_controller_test="database::container_controller"
plant_controller_test="database::plant_controller"

# run tests
cargo test --release $cli_test 2> $err_redirrect
cargo test --release $garden_controller_test 2> $err_redirrect
cargo test --release $container_controller_test 2> $err_redirrect
cargo test --release $plant_controller_test 2> $err_redirrect