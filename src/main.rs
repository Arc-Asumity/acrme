// Copyright (c) 2025 Arc Asumity
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/main.rs
// Entry point of the arcme ACME client.

mod arg;

fn main() {
    init();
}

fn init() {
    let parsed_args: arg::InputArgs = match arg::parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            std::process::exit(1);
        }
    };
}
