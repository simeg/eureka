# eureka [![Crate Status](https://img.shields.io/crates/v/eureka.svg)](https://crates.io/crates/eureka)  [![Build Status](https://travis-ci.com/simeg/eureka.svg?branch=master)](https://travis-ci.com/simeg/eureka)
`eureka` is a CLI tool that allows you to quickly write down your idea with the editor of your
choice, and then have the idea committed and pushed to your idea git repository.

Imagine working on something important and then having an idea. Instead of letting your idea
slip by you can just type `eureka` and you're able to quickly store your idea and then
continue working.

**It's developed _by_ and _for_ people who enjoy using the terminal.**

![demo](assets/demo.gif)

## Required Setup
`eureka` requires a git repository with a `README.md` in the root folder. This is the default
structure when you create an empty repository with a readme on GitHub, so it's easy to start using
it. And since it's your own repository you can make it private to keep your ideas secret.

## Installation
```bash
$ cargo install eureka
```

### Install via Homebrew

```bash
$ brew install eureka
```

Use the same command to update `eureka` to the latest version.

## Usage
The first time you run `eureka` it will ask a few questions regarding your setup. This configuration
will be stored in `~/.eureka/`.

After the setup is complete you simply run:

```bash
$ eureka
```

When your idea is committed it's pushed to `origin/master`.

To view your stored ideas use the `-v` or `--view` flag.

```bash
$ eureka --view
```

### Flags

```bash
--clear-editor    Clear the stored path to your idea editor
--clear-repo      Clear the stored path to your idea repo
-v, --view        View your ideas using less
```

### Recommended alias
An easy to remember alias for `eureka` is the word `idea`. This makes it easy to remember to use
`eureka` to store your ideas.

**Zsh**
```sh
echo 'alias idea="eureka"' >> ~/.zshrc
```

**Bash**
```sh
echo 'alias idea="eureka"' >> ~/.bashrc
```

## Development

This repo uses a Makefile as an interface for common operations.

1) Do code changes
2) Run `make build link` to build the project and create a symlink from the built binary to the root
   of the project
3) Run `./eureka` to execute the binary with your changes
4) Profit :star:

## Improvements
See [github issues](https://github.com/simeg/eureka/issues).
