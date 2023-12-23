/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */

#[derive(Clone, Debug, PartialEq)]
pub struct FilePath(pub Vec<String>);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyPath(pub Vec<String>);
