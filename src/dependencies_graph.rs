use crate::{dependencies::DependencyPath, trie::Trie};

/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */

/**
 * At each node, the value represents the full path from the root, and the dependencies read at this node.
 */
pub type DependenciesGraph = Trie<String, Vec<DependencyPath>>;
