# {{project-name}} CLI

{{project-description}}

## What does {{project-name}}?

In short, {{project-name}} solves the problem of ...

## Quick Start

### Install

To install the {{project-name}}, you just need to run

```bash
cargo install --force {{project-name}}
```

(--force just makes it update to the latest `{{project-name}}` if it's already installed)

**Note** the binary is called `{{project-name | split: "-cli" | first}}` (without `-cli`)

to verify if the installation was successful, you can run `which {{project-name | split: "-cli" | first}}` that should output similar to

```sh
$HOME/.cargo/bin/{{project-name | split: "-cli" | first}}
```

### Usage

```sh
$ {{project-name | split: "-cli" | first}} --help

TODO: --help output goes here
```

## License

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

- **[GNU GPL v3 license](https://www.gnu.org/licenses/gpl-3.0)**
- Copyright 2019 © [Sven Assmann][me].

[me]: https://www.d34dl0ck.me
