// Copyright (c) 2025 Arc Asumity
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/arg.rs
// This module handles command-line argument parsing.

use std::env;

pub struct InputArgs {
    pub config_path: Option<String>,
}

pub fn parse_args() -> Result<InputArgs, String> {
    let mut args: std::iter::Skip<std::env::Args> = env::args().skip(1);
    let mut config_path: Option<String> = None;

    while let Some(arg) = args.next() {
        if arg == "-v" || arg == "--version" {
            println!("Arcme ACME Client Version {}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        } else if arg == "-h" || arg == "--help" || arg == "?" {
            println!("{}", include_str!("../text/help.txt"));
            std::process::exit(0);
        } else {
            if arg.starts_with("-") {
                return Err(format!("Unknown argument: {}", arg));
            } else {
                if config_path.is_none() {
                    config_path = Some(arg);
                } else {
                    return Err(format!("Multiple path arguments detected: {}", arg));
                }
            }
        }
    }

    Ok(InputArgs { config_path })
}
