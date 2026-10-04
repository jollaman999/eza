<!--
SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->
# Installation

This fork's one-letter options match GNU `ls`. The `eza` packages in Linux
distributions, Homebrew, MacPorts, Nixpkgs, Winget, Scoop and crates.io are
upstream eza and do not have these options, so install it in one of the ways
below.

### Manual (Linux)

Releases ship x86_64 Linux builds, with and without Git support
(`eza_x86_64-unknown-linux-gnu_no_libgit.tar.gz`).

```shell
wget -c https://github.com/jollaman999/eza/releases/latest/download/eza_x86_64-unknown-linux-gnu.tar.gz -O - | tar xz
sudo chmod +x eza
sudo chown root:root eza
sudo mv eza /usr/local/bin/eza
```

The man pages and shell completions are attached to each release as
`man-<version>.tar.gz` and `completions-<version>.tar.gz`.

### Cargo (git)

If you already have a Rust environment set up, you can let Cargo build it from this repository:

    cargo install --git https://github.com/jollaman999/eza

or from a local clone:

    git clone https://github.com/jollaman999/eza.git
    cd eza
    cargo install --path .

Cargo will build the `eza` binary and place it in `$HOME/.cargo`.
To build without Git support, add `--no-default-features`.

### Nix (Linux, MacOS)

The flake in this repository builds this fork:

```shell
nix profile install github:jollaman999/eza
```

### Using it as `ls`

```shell
alias ls='eza --icons=auto'
```

`--icons=auto` shows icons only when the output is a terminal, so pipes and
`$(ls)` get plain names.

### Completions

#### For zsh:

> **Note**
> Change `~/.zshrc` to your preferred zsh config file.

##### Clone the repository:

```sh
git clone https://github.com/jollaman999/eza.git
```

##### Add the completion path to your zsh configuration:

Replace `<path_to_eza>` with the actual path where you cloned the `eza` repository.

```sh
echo 'export FPATH="<path_to_eza>/completions/zsh:$FPATH"' >> ~/.zshrc
```

##### Reload your zsh configuration:

```sh
source ~/.zshrc
```


#### For zsh with homebrew:

In case zsh completions don't work out of the box with homebrew, add the
following to your `~/.zshrc`:

```bash
if type brew &>/dev/null; then
    FPATH="$(brew --prefix)/share/zsh/site-functions:${FPATH}"
    autoload -Uz compinit
    compinit
fi
```

For reference:
- https://docs.brew.sh/Shell-Completion#configuring-completions-in-zsh
- https://github.com/Homebrew/brew/issues/8984
