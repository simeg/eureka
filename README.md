# eureka [![Crate Status](https://img.shields.io/crates/v/eureka.svg)](https://crates.io/crates/eureka)  [![Build Status](https://travis-ci.com/simeg/eureka.svg?branch=master)](https://travis-ci.com/simeg/eureka)
`eureka` is a CLI tool that allows you to quickly write down an idea using your
preferred editor, and then have the idea committed and pushed to your idea
git repository.

Imagine working on something important and then having an idea. Instead of
letting your idea slip by you can just type `eureka` and you're able to quickly
store your idea and then continue working.

**It is developed _by_ and _for_ people who enjoy using the terminal.**

![demo](assets/demo.gif)

## Required Setup
`eureka` requires a git repository with a `README.md` in the root folder. This
is the default structure when you create an empty repository with a readme on
GitHub, so it's easy to start using it. And since it's your own repository you
can make it private to keep your ideas secret.

## Installation

**[Homebrew](https://brew.sh/)**
```sh
$ brew install eureka
```

**[Cargo](https://doc.rust-lang.org/cargo)**
```sh
$ cargo install eureka
```

## Usage
The first time you run `eureka` it will ask a few questions regarding your
setup. This configuration will be stored in `~/.eureka/`.

After the setup simply run `eureka` to capture an idea. It will then be 
committed and pushed to `origin/master`.

View your stored ideas in `$PAGER` env variable (falls back to `less`) with the
`-v` or `--view` flag.

```sh
$ eureka --view
```

### Flags

```sh
--clear-editor    Clear the stored path to your idea editor
--clear-repo      Clear the stored path to your idea repo
-v, --view        View ideas with your $PAGER env variable. If unset use less
```

### Recommended alias
An easy to remember alias for `eureka` is the word `idea`. This makes it easy
to remember to use `eureka` to store your ideas.

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
