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
    arg::handle_cli();
}
