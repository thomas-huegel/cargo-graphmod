<!--
 Copyright 2023 Thomas Hügel.
 This file is part of Cargo Graphmod.
 SPDX-License-Identifier: GPL-3.0-only
-->


# cargo-graphmod

A `cargo` subcommand for building Graphviz DOT files of dependency graphs between the modules of a package.


## Prerequisites

You need to install [Graphviz](https://graphviz.org/).


## Installing

`cargo-graphmod` can be installed with `cargo install`:

```
$ cargo install cargo-graphmod
```

## Usage

```
$ cd my_rust_package
$ cargo graph | dot -Tpdf > modules.pdf
```

## Known limitations

* Detects only dependencies introduced by the `use` keyword.
* Works best when there is a bijection between modules and files / directories.


## License

`cargo-graphmod` is released under the terms of the GPLv3 license.


## Cargo Graphmod's graph

![cargo-graphmod](modules.pdf)