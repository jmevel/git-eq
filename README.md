# `git-eq` (aka *git earthquake*)

Earthquakes are part of the daily life in many countries like in Taiwan.
`git-eq` is a simple git command to quickly save your local changes in case of an emergency like this.
This project is heavily inspired by [git-fire](https://github.com/qw3rtman/git-fire).

## What this command does

1. `Checkout` to a new branch named `earthquake/<origin-branch>-<email>-<elapsed-seconds-since-unix-epoch>` (eg: *`earthquake/master-bob@domain.com-1652438295`*)
2. If there are some uncommited changes
   1. `Add` all those files (even if you're not in the root directory)
   2. `Commit` with either the default message or the provided one
3. `Push`

## Installation

```sh
cargo install git-eq
```

## Usage

```sh
git eq [message]
```

### Examples

```sh
git eq
```

Will produce a commit with the default message `Earthquake!!! This is an emergency commit`

```sh
git eq "My custom message"
```

Will produce a commit with the message `My custom message`
