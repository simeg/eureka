# Changelog

## Version 2.0.0

This version introduces some breaking changes for _how_ and _where_ configuration is stored, as well as changes the default git branch from `master` to `main`, and making the branch name non-configurable.

⚠️ **Breaking changes** ⚠️
* Config is now stored as JSON
* Config is now stored at `$XDG_CONFIG_HOME`, if unset use `$HOME/.config/eureka`. This is more
  inline with where config should be stored
* `main` branch is used by default, and you can't config the branch name anymore. Use `git checkout main` to create a new branch based on our current one.

Just update and run it to go through the first time setup again.

**Other changes**
* Only allow absolute path for repo
* Only allow to clear entire config instead of just repo (`--clear-config`)
* Dependency and docs updates

## Version 1.8.1
* Bugfix: Commit and push to stored branch name

## Version 1.8.0
* Use git2-rs crate for running git commands
* Output to stdout is changed (because of that ^)

## Version 1.7.0
* Use $EDITOR to edit ideas, fall back to vi
* Use $PAGER to view ideas, fall back to less

## Version 1.6.3
* Refactor code to be more idiomatic and readable
* Only remove repo/editor if they exist
* Verify chosen editor exists on $PATH before finish setup
* Update deps to latest versions

## Version 1.6.2
* Include `Cargo.lock` in binary to make it possible to distribute via Homebrew

## Version 1.6.1
* If user inputs empty repo path - ask again
* Make binary resolution work on Windows too

## Version 1.6.0
* Remove unused `text_io` dependency

## Version 1.5.2
* Use path to vim that's compatible with more operating systems
* Add shorthand flag (`-v`) for viewing stored ideas
* Trim all user input

## Version 1.5.1
* Abort early if any clear flag is present

## Version 1.5.0
* Add colors to output

## Version 1.4.0
* Add flag `--view` for viewing stored ideas.

## Version 1.3.0
* Remove usage of numbers in selecting your editor which caused confusion.
* After first time setup - do not prompt user for idea.

## Version 1.2.0
* The dependency `serde` has been removed. I think it deserves a new release.

## Version 1.1.0
* Flags `--clear-editor` and `--clear-repo` now actually works

