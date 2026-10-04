<!--
SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->

<div align="center">
    
# eza

A modern replacement for ls.

This is a fork of [eza-community/eza](https://github.com/eza-community/eza) whose one-letter options mean the same as GNU `ls`, so it can stand in for `ls`.

[![Built with Nix](https://img.shields.io/badge/Built_With-Nix-5277C3.svg?logo=nixos&labelColor=73C3D5)](https://nixos.org)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

[![Unit tests](https://github.com/jollaman999/eza/actions/workflows/unit-tests.yml/badge.svg)](https://github.com/jollaman999/eza/actions/workflows/unit-tests.yml)

</div>

![eza demo gif](docs/images/screenshots.png)

---

**eza** is a modern alternative for the venerable file-listing command-line program `ls` that ships with Unix and Linux operating systems, giving it more features and better defaults.
It uses colours to distinguish file types and metadata.
It knows about symlinks, extended attributes, and Git.
And it’s **small**, **fast**, and just **one single binary**.

By deliberately making some decisions differently, eza attempts to be a more featureful, more user-friendly version of `ls`.

---

**eza** features not in exa (non-exhaustive):

- Fixes [“The Grid Bug”](https://github.com/eza-community/eza/issues/66#issuecomment-1656758327) introduced in exa 2021.
- Hyperlink support.
- Mount point details.
- Selinux context output.
- Git repo status output.
- Human readable relative dates.
- Several security fixes.
- Support for `bright` terminal colours.
- Many smaller bug fixes/changes!
- Configuration `theme.yml` file for customization of colors and icons.

...and like, so much more that it became exhausting to update this all the time.
Like seriously, we have a lot of good stuff.

---

<a id="try-it">
<h1>Try it!</h1>
</a>

### Nix ❄️

If you already have Nix setup with flake support, you can try out eza with the `nix run` command:

    nix run github:jollaman999/eza

Nix will build eza and run it.

If you want to pass arguments this way, use e.g. `nix run github:jollaman999/eza -- -ol`.

# Installation

The `eza` packages in distributions, Homebrew, Winget and crates.io are upstream eza, which does not have the `ls`-compatible options. Install this fork from its [releases](https://github.com/jollaman999/eza/releases) or build it with Cargo, as described in [INSTALL.md](INSTALL.md).

---

<a id="options">
<h1>Command-line options</h1>
</a>

eza’s one-letter options mean the same as GNU `ls`’s. Options that only eza has, and whose letter would clash with an `ls` option, have a long name only. Quick overview:

## Meta options

<details>
<summary>Click to expand</summary>

- **-?**, **--help**: show list of command-line options
- **--version**: show version of eza
- **--stdin**: read file names from stdin; like `ls`, eza never reads stdin without this option

</details>

## Display options

<details>
<summary>Click to expand</summary>

- **-1**, **--oneline**: display one entry per line (does not turn off a long view)
- **-l**, **--long**: display extended details and attributes
- **-g**, **--long-no-owner**: like `-l`, but hide the user and list the group
- **-o**, **--long-no-group**: like `-l`, but hide the group
- **-n**, **--numeric-uid-gid**: like `-l`, but list numeric user and group IDs
- **-C**, **--format-columns**: display entries as a grid, down columns (default)
- **-x**, **--across**: display entries as a grid, across rows
- **--grid**: display entries as a grid (with a long view option, a grid of tables)
- **-R**, **--recurse**, **--recursive**: recurse into directories
- **--tree**: recurse into directories as a tree
- **--level=(depth)**: limit the depth of recursion
- **--code[=(mode)]**: summarise lines of code by language instead of listing files (lines, percent, both)
- **--follow-symlinks**: drill down into symbolic links that point to directories
- **-F**: display type indicator by file names (same as `--classify=always`)
- **--classify[=(when)]**: display type indicator by file names (always, auto, never; without a value, always)
- **-L**, **--dereference**: dereference symlinks for file information
- **--colo[u]r=(when)**: when to use terminal colours (always, auto, never)
- **--color-scale=(field)**: highlight levels of `field` distinctly(all, age, size)
- **--color-scale-mode=(mode)**: use gradient or fixed colors in --color-scale. valid options are `fixed` or `gradient`
- **--icons=(when)**: when to display icons (always, auto, never)
- **--hyperlink=(when)**: when to display entries as hyperlinks (always, auto, never)
- **--absolute=(mode)**: display entries with their absolute path (on, follow, off)
- **--no-quotes**: don't quote file names with spaces
- **--short-nix**: abbreviate Nix store hashes in file names and paths
- **-w**, **--width=(columns)**: set screen width in columns (0 means no limit; it does not turn on the grid when the output is not a terminal)

Like `ls`, the last of `-l`, `-g`, `-o`, `-n`, `-C`, and `-x` picks between the table and the grid.

</details>

## Filtering and sorting options

<details>
<summary>Click to expand</summary>

- **-a**, **--all**: show hidden and 'dot' files, and the `.` and `..` directories (with `--tree`, only hidden files)
- **-A**, **--almost-all**: show hidden and 'dot' files, but not `.` and `..`
- **-f**, **--unsorted-all**: same as `-a -U`
- **-d**, **--treat-dirs-as-files**, **--directory**: list directories like regular files (wins over `-R`, so `-dR` lists the directories themselves)
- **-r**, **--reverse**: reverse the sort order
- **--sort=(field)**: which field to sort by
- **-t**, **--sort-time**: sort by time, newest first (the time field is picked by `-c`, `-u`, or `--time`)
- **-S**, **--sort-size**: sort by file size, largest first
- **-U**, **--unsorted**: do not sort; list entries in directory order
- **-X**, **--sort-extension**: sort by file extension
- **-v**, **--sort-version**: sort by name, with numbers in natural order
- **-c**, **--ctime**: use the changed time; show it with `-l`, sort by it with `-t` or without `-l`
- **-u**, **--atime**: use the accessed time; show it with `-l`, sort by it with `-t` or without `-l`
- **--group-directories-first**: list directories before other files
- **--group-directories-last**: list directories after other files
- **--only-dirs**: list only directories
- **--only-files**: list only files
- **--no-symlinks**: don't show symbolic links
- **--show-symlinks**: explicitly show links (with `--only-dirs`, `--only-files`, to show symlinks that match the filter)
- **--git-ignore**: ignore files mentioned in `.gitignore`
- **-I**, **--ignore-glob=(globs)**, **--ignore=(globs)**: glob patterns (pipe-separated) of files to ignore; can be given more than once
- **-B**, **--ignore-backups**: ignore files whose names end with `~`

Files with the same size or time are sorted by name.

</details>

## Long view options

<details>
<summary>Click to expand</summary>

These options are available when running with `--long` (`-l`). `-g`, `-o`, and `-n` also turn on the long view, and `-s` and `-Z` also work without it, putting their value before each file name like `ls`:

- **-h**, **--human-readable**: list file sizes with binary prefixes (same as `--binary`)
- **--binary**: list file sizes with binary prefixes
- **--bytes**: list file sizes in bytes, without any prefixes
- **--group**: list each file’s group
- **-G**, **--no-group**: hide the group (overrides `-g` and `--group`)
- **--smart-group**: only show group if it has a different name from owner
- **--numeric**: list numeric user and group IDs
- **--header**: add a header row to each column
- **--links**: list each file’s number of hard links
- **-i**, **--inode**: list each file’s inode number
- **--loc[=(mode)]**: add lines-of-code and language columns (lines, percent, both)
- **--modified**: use the modified timestamp field
- **-M**, **--mounts**: Show mount details (Linux and MacOS only).
- **-s**, **--size**: show size of allocated file system blocks (same as `--blocksize`)
- **--blocksize**: show size of allocated file system blocks
- **--time=(field)**: which timestamp field to use (also sorts by it with `-t` or without `-l`)
- **--accessed**: use the accessed timestamp field
- **--created**: use the created timestamp field
- **-O**, **--flags**: list file flags (Mac, BSD, and Windows only)
- **-Z**, **--context**: list each file’s security context
- **-@**, **--extended**: list each file’s extended attributes and sizes
- **--changed**: use the changed timestamp field
- **--git**: list each file’s Git status, if tracked or ignored
- **--git-repos**: list each directory’s Git status, if tracked
- **--git-repos-no-status**: list whether a directory is a Git repository, but not its status (faster)
- **--no-git**: suppress Git status (always overrides `--git`, `--git-repos`, `--git-repos-no-status`)

- **--time-style**: how to format timestamps. valid timestamp styles are ‘`default`’, ‘`iso`’, ‘`long-iso`’, ‘`full-iso`’, ‘`relative`’, or a custom style ‘`+<FORMAT>`’ (E.g., ‘`+%Y-%m-%d %H:%M`’ => ‘`2023-09-30 13:00`’. For more specifications on the format string, see the _`eza(1)` manual page_ and [chrono documentation](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).).
- **--total-size**: show recursive directory size
- **--no-permissions**: suppress the permissions field
- **--octal-permissions**: list each file's permission in octal format
- **--no-filesize**: suppress the filesize field
- **--no-user**: suppress the user field
- **--no-time**: suppress the time field

Some of the options accept parameters:

- Valid **--colo\[u\]r** options are **always**, **automatic** (or **auto** for short), and **never**.
- Valid sort fields are **accessed**, **changed**, **created**, **extension**, **Extension**, **inode**, **modified**, **name**, **Name**, **.name**, **.Name**, **size**, **time**, **version**, **type**, and **none**. Fields starting with a capital letter sort uppercase before lowercase. Like `ls`, **size** puts the largest files first and **time** puts the newest files first; **version** is the same as **name**, and **width** gives an error. The modified field, which puts the oldest first, has the aliases **date** and **newest**, while its reverse has the aliases **age** and **oldest**.
- Valid time fields are **modified**, **changed**, **accessed**, and **created**, and the `ls` words **mtime**, **modification**, **ctime**, **status**, **atime**, **access**, **use**, **birth**, and **creation**.
- Valid time styles are **default**, **iso**, **long-iso**, **full-iso**, and **relative**.

### Unsupported `ls` options

These `ls` options give a "not supported" error instead of being silently ignored: **-T** (**--tabsize**), **-D** (**--dired**), **-H** (**--dereference-command-line**), **-m** (**--format-commas**), **-b** (**--escape**), **-Q** (**--quote-name**), **-N** (**--literal**), **-q** (**--hide-control-chars**), **-p** (**--indicator-slash**), and **-k** (**--kibibytes**).



See the `man` pages for further documentation of usage. They are available
- online [in the repo](https://github.com/jollaman999/eza/tree/main/man)
- in your terminal via `man eza`, as of version [`[0.18.13] - 2024-04-25`](https://github.com/jollaman999/eza/blob/main/CHANGELOG.md#01813---2024-04-25)
</details>


## Custom Themes
<details>
<summary>Click to expand</summary>

**Eza** has recently added support for a `theme.yml` file, where you can specify all of the existing theme-ing options
available for the `LS_COLORS` and `EXA_COLORS` environment variables, as well as the option to specify different icons
for different file types and extensions. Any existing environment variables set will continue to work and will take
precedence for backwards compatibility.

#### **New** Pre-made themes
Check out the themes available in the official [eza-themes](https://github.com/eza-community/eza-themes) repository, or contribute your own.

An example theme file is available in `docs/theme.yml`, and needs to either be placed in a directory specified by the 
environment variable `EZA_CONFIG_DIR`, or will looked for by default in `$XDG_CONFIG_HOME/eza`.

Full details are available on the [man page](https://github.com/jollaman999/eza/tree/main/man/eza_colors-explanation.5.md) and an example theme file is included [here](https://github.com/jollaman999/eza/tree/main/docs/theme.yml)

</details>


# Hacking on eza

If you wanna contribute to eza, firstly, you're expected to follow our 
[code of conduct](https://github.com/jollaman999/eza/blob/main/CODE_OF_CONDUCT.md). 
After having understood the code of conduct, you can have a look at our
[CONTRIBUTING.md](https://github.com/jollaman999/eza/blob/main/CONTRIBUTING.md) 
for more info about actual hacking.

