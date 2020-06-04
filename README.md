# {{project-name}} CLI

TODO: a useful minimal description would be nice

## What does {{project-name}}?

In short, {{project-name}} solves the problem of ...

## Quick Start

### Install

To install the {{project-name}}, you just need to run

```bash
cargo install --force {{crate_name}}
```

(--force just makes it update to the latest `{{crate_name}}` if it's already installed)

_Note_ the binary is called `{{crate_name | split: "-cli" | first}}` (without `-cli`)

to verify if the installation was successful, you can run `which {{crate_name | split: "-cli" | first}}` that should output similar to

```sh
$HOME/.cargo/bin/{{crate_name | split: "-cli" | first}}
```

### Usage

```sh
$ {{crate_name | split: "-cli" | first}} --help

TODO: all the out put goes here
```

## License

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

- **[GNU GPL v3 license](https://www.gnu.org/licenses/gpl-3.0)**
- Copyright 2019 © [Sven Assmann][me].

[me]: https://www.d34dl0ck.me
