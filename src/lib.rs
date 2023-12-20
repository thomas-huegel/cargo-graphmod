/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */

#[doc = include_str!("../README.md")]
pub mod app_builder;
mod colors;
mod dependencies_graph;
mod dependencies_parser;
mod dependency_components;
mod dot_formatter;
mod files_reader;
mod trie;
