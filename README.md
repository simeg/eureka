# eureka [![Crate Status](https://img.shields.io/crates/v/eureka.svg)](https://crates.io/crates/eureka)
`eureka` is a CLI tool that allows you to quickly write down your idea with the editor of your
choice, and then have the idea committed and pushed to your idea git repository.

Imagine working on something important and then having an idea. Instead of letting your idea
slip by you can just type `eureka` and you're able to quickly store your idea and then
continue working.

**It's developed _by_ and _for_ people who likes living in the terminal.**

![demo](assets/demo.gif)

## Required Setup
`eureka` requires a git repository with a `README.md` in the root folder. This is the default
structure when you create an empty repository with a readme on GitHub, so it's easy to start using
it. And since it's your own repository you can make it private to keep your ideas secret.

### Dependencies
- [rust](https://www.rust-lang.org)
- [cargo](https://crates.io/)

## Installation
```bash
$ cargo install eureka
```

## Usage
The first time you run `eureka` it will ask a few questions regarding your setup. This configuration
will be stored in `~/.eureka/`.

After the setup is complete you simply run:

```bash
$ eureka
```

When your idea is committed it's pushed to `origin/master`.

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

## Improvements
See [github issues](https://github.com/simeg/eureka/issues).

## License
MIT
