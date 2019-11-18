// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct SafetyRulesConfig {
    pub backend: SafetyRulesBackend,
}

impl Default for SafetyRulesConfig {
    fn default() -> Self {
        Self {
            backend: SafetyRulesBackend::InMemoryStorage,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SafetyRulesBackend {
    InMemoryStorage,
    OnDiskStorage {
        // In testing scenarios this implies that the default state is okay if
        // a state is not specified.
        default: bool,
        // Required path for on disk storage
        path: PathBuf,
    },
}
