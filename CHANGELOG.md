# Changelog

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

