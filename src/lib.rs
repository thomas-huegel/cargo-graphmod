/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */

#[doc = include_str!("../README.md")]
pub mod app_builder;
mod dependencies;
mod dependencies_graph;
mod dependencies_processor;
mod files_reader;
mod formatter;
mod parser;
mod trie;
