// Copyright (c) 2025 Arc Asumity
// Licensed under the GPLv3 or later License.
// See LICENSE file to details.
//
// src/conf.rs
// This module handles configuration file parsing and loading.

use std::collections::{HashMap, HashSet};
use serde::Deserialize;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
enum CertContent {
    cert-only,
    key-only,
    cert-key,
    cert-chain-only,
    cert-chain-key,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
enum CertType {
    PEM,
    DER,
    PKCS7,
    PKCS12,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
struct AcmeServer {
    host: String,
    port: u16,
    url: String,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(untagged)]
enum ConfigAcmeServer {
    Ref(String),
    Inline(AcmeServer),
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
enum ChallengeType {
    Http01, // TODO HTTP-01 Chanllenge
    Dns01, // TODO DNS-01 Chanllenge
    TlsAlpn, // TODO TLS-ALPN Chanllenge
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
struct CentOutput {
    path: String,
    cert_type: CertType,
    cert_content: CertContent,
    from_server: ConfigAcmeServer,
    chanllenge_type: ChallengeType,
}

#[derive(Deserialize, Debug)] 
struct ConfigFile {
    acme_servers: Option<HashMap<String, AcmeServer>>,
    cert_output: HashSet<CentOutput>,
}

fn parse_config(config_path: String) -> Result<ConfigFile, String> {
    match std::fs::read_to_string(path) {
        Ok(json_str) => {
            match serde_json::from_str(json_str) {
                Ok(parsed_config) => Ok(parsed_config),
                Err(e) => Err(format!("Deserialization error: {}", e))
            }
        }
        Err(e) => Err(format!("Failed read configuration file: {}", e))
    }
}
