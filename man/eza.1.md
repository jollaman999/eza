% eza(1) $version

<!-- This is the eza(1) man page, written in Markdown. -->
<!-- To generate the roff version, run `just man`, -->
<!-- and the man page will appear in the ‘target’ directory. -->


NAME
====

eza — a modern replacement for ls


SYNOPSIS
========

`eza [options] [files...]`

**eza** is a modern replacement for `ls`.
It uses colours for information by default, helping you distinguish between many types of files, such as whether you are the owner, or in the owning group.

It also has extra features not present in the original `ls`, such as viewing the Git status for a directory, or recursing into directories with a tree view.

The one-letter options mean the same as those of GNU `ls`. Options that only eza has, and whose letter would clash with an `ls` option, have a long name only. `ls` options that eza does not support give an error instead of being silently ignored; see **UNSUPPORTED LS OPTIONS**.


EXAMPLES
========

`eza`
: Lists the contents of the current directory in a grid.

`eza --oneline --sort=size`
: Displays a list of files with the largest at the top.

`eza -lt`
: Displays a table of files with the most recently modified at the top.

`eza --long --header --inode --git`
: Displays a table of files with a header, showing each file’s metadata, inode, and Git status.

`eza --long --tree --level=3`
: Displays a tree of files, three levels deep, as well as each file’s metadata.


META OPTIONS
===============

`-?`, `--help`
: Show list of command-line options.

`--version`
: Show version of eza.

`--stdin`
: Read the file names to list from standard input, one per line, or separated by the character in the `EZA_STDIN_SEPARATOR` environment variable.
Like `ls`, eza does not read standard input unless this option is given, even when standard input is not a terminal.

DISPLAY OPTIONS
===============

`-1`, `--oneline`
: Display one entry per line.
This does not turn off a long view option given with it.

`-l`, `--long`
: Display extended file metadata as a table.

`-g`, `--long-no-owner`
: Like `-l`, but do not list each file’s user, and list each file’s group.

`-o`, `--long-no-group`
: Like `-l`, but do not list each file’s group.

`-n`, `--numeric-uid-gid`
: Like `-l`, but list numeric user and group IDs.

`-C`, `--format-columns`
: Display entries as a grid, listed down columns (default).

`-x`, `--across`
: Display entries as a grid, listed across rows rather than down columns.

`--grid`
: Display entries as a grid (default).
When given with a long view option, display the tables side by side in a grid.

Like `ls`, the last of `-l`, `-g`, `-o`, `-n`, `-C`, and `-x` picks between the table and the grid, so ‘`eza -lC`’ shows a grid and ‘`eza -Cl`’ shows a table.

`-R`, `--recurse`, `--recursive`
: Recurse into directories.
Given with `-d`, `-d` wins and the directories are listed as files.

`--tree`
: Recurse into directories as a tree.

`--level=DEPTH`
: Limit the depth of recursion.

`--absolute=WHEN`
: Display entries with their absolute path.

Valid settings are '`on`', '`follow`', and '`off`'.
When used without a value, defaults to '`on`'.

'`on`': Show absolute paths for all entries.
'`follow`': Show absolute paths and resolve symbolic links to their targets.
'`off`': Show relative paths (default behavior).

`-F`
: Display file kind indicators next to file names, like `ls -F`.
The same as `--classify=always`. This option takes no value.

`--classify[=WHEN]`
: Display file kind indicators next to file names.

Valid settings are ‘`always`’, ‘`automatic`’ (or ‘`auto`’ for short), and ‘`never`’.
When used without a value, defaults to ‘`always`’, like `ls --classify`. A value must be joined with ‘`=`’.

`automatic` or `auto` will display file kind indicators only when the standard output is connected to a real terminal. If `eza` is ran while in a `tty`, or the output of `eza` is either redirected to a file or piped into another program, file kind indicators will not be used. Setting this option to ‘`always`’ causes `eza` to always display file kind indicators, while ‘`never`’ disables the use of file kind indicators.

`--code[=MODE]`
: Print a lines-of-code summary by language instead of listing files, in the spirit of tools like `tokei` and `cloc`.

: The given paths (or the current directory) are walked recursively, honouring a git repository’s `.gitignore` when one is present, and each recognised language is reported with its file, line, code, comment, and blank counts, plus a bar visualising its share of the code. Valid modes are ‘`lines`’, ‘`percent`’, and ‘`both`’ (the default).

`--follow-symlinks`
: Drill down into symbolic links that point to directories.

`-L`, `--dereference`
: Dereference symbolic links when displaying information: show the information of the file a link points to, rather than of the link itself.

`--color=WHEN`, `--colour=WHEN`
: When to use terminal colours (using ANSI escape code to colorize the output).

Valid settings are ‘`always`’, ‘`automatic`’ (or ‘`auto`’ for short), and ‘`never`’.
When used without a value, defaults to ‘`automatic`’.

The default behavior (‘`automatic`’ or ‘`auto`’) is to colorize the output only when the standard output is connected to a real terminal. If the output of `eza` is redirected to a file or piped into another program, terminal colors will not be used. Setting this option to ‘`always`’ causes `eza` to always output terminal color, while ‘`never`’ disables the use of terminal color.

Manually setting this option overrides `NO_COLOR` environment.

`--color-scale`
: highlight levels of `field` distinctly.
Use comma(,) separated list of all, age, size

`--color-scale-mode=MODE`
: Use gradient or fixed colors in `--color-scale`.

Valid options are `fixed` or `gradient`.
When used without a value, defaults to `gradient`.

`--icons=WHEN`
: Display icons next to file names.

Valid settings are ‘`always`’, ‘`automatic`’ (‘`auto`’ for short), and ‘`never`’.
When used without a value, defaults to ‘`automatic`’.

`automatic` or `auto` will display icons only when the standard output is connected to a real terminal. If `eza` is ran while in a `tty`, or the output of `eza` is either redirected to a file or piped into another program, icons will not be used. Setting this option to ‘`always`’ causes `eza` to always display icons, while ‘`never`’ disables the use of icons.

`--no-quotes`
: Don't quote file names with spaces.

`--short-nix`
: Abbreviate Nix store hashes in file names and paths.

: A path component beginning with a Nix store hash — exactly 32 characters of Nix’s base32 alphabet followed by a dash, like `vlkia5wk0svsikwv50554mh06iayg2m2-source.drv` — is displayed with the hash shortened to its first 8 characters and an ellipsis, painted dim so the name stands out: `vlkia5wk…-source.drv`. This applies to listed names, symbolic link targets, and absolute paths.

`--hyperlink=WHEN`
: Display entries as hyperlinks

Valid settings are ‘`always`’, ‘`automatic`’ (‘`auto`’ for short), and ‘`never`’.
When used without a value, defaults to ‘`automatic`’.

`automatic` or `auto` will display hyperlinks only when the standard output is connected to a real terminal. If `eza` is ran while in a `tty`, or the output of `eza` is either redirected to a file or piped into another program, hyperlinks will not be used. Setting this option to ‘`always`’ causes `eza` to always display hyperlinks, while ‘`never`’ disables the use of hyperlinks.

`-w`, `--width=COLS`
: Set screen width in columns, overriding the `COLUMNS` environment variable.
A width of 0 means there is no limit.
Like `ls -w`, this only sets the width: when the output is not a terminal, it does not turn on the grid by itself. Use `-C` or `-x` for that.

FILTERING AND SORTING OPTIONS
=============================

`-a`, `--all`
: Show hidden and “dot” files, and the ‘`.`’ and ‘`..`’ directories.
With `--tree`, only the hidden files are shown. Giving it more than once is the same as giving it once.

`-A`, `--almost-all`
: Show hidden and “dot” files, but not the ‘`.`’ and ‘`..`’ directories.

`-f`, `--unsorted-all`
: The same as `-a -U`: show all files, and do not sort.

`-d`, `--treat-dirs-as-files`, `--directory`
: This flag, inherited from `ls`, changes how `eza` handles directory arguments.

: Instead of recursing into directories and listing their contents (the default behavior), it treats directories as regular files and lists information about the directory entry itself.

: This is useful when you want to see metadata about the directory (e.g., permissions, size, modification time) rather than its contents.

: For simply listing only directories and not files, consider using the `--only-dirs` option as an alternative.

: When given with `-R` (as in ‘`eza -dR`’), `-d` wins and the directories are not recursed into, like `ls -dR`.

`-r`, `--reverse`
: Reverse the sort order.

`--sort=SORT_FIELD`
: Which field to sort by.

Valid sort fields are ‘`name`’, ‘`Name`’, ‘`.name`’, ‘`.Name`’, ‘`extension`’, ‘`Extension`’, ‘`size`’, ‘`time`’, ‘`version`’, ‘`modified`’, ‘`changed`’, ‘`accessed`’, ‘`created`’, ‘`inode`’, ‘`type`’, and ‘`none`’.

Like `ls`, ‘`size`’ puts the largest files first, and ‘`time`’ puts the newest files first, using the time field picked by `-c`, `-u`, or `--time` (the modified time by default). ‘`version`’ is the same as ‘`name`’, which already sorts numbers within names in natural order. ‘`none`’ does not sort. ‘`width`’, which `ls` accepts, is not supported and gives an error.

The `modified` sort field, which puts the oldest files first, has the aliases ‘`date`’ and ‘`newest`’, and its reverse order has the aliases ‘`age`’ and ‘`oldest`’.

Sort fields starting with a capital letter will sort uppercase before lowercase: ‘A’ then ‘B’ then ‘a’ then ‘b’. Fields starting with a lowercase letter will mix them: ‘A’ then ‘a’ then ‘B’ then ‘b’.

When sorting by size or by a time field, files with the same size or time are sorted by name.

`-t`, `--sort-time`
: Sort by time, newest first. The time field is the modified time, or the one picked by `-c`, `-u`, or `--time`.

`-S`, `--sort-size`
: Sort by file size, largest first.

`-U`, `--unsorted`
: Do not sort; list entries in directory order.

`-X`, `--sort-extension`
: Sort alphabetically by file extension.

`-v`, `--sort-version`
: Sort by name, with numbers within names in natural (version) order.

`-c`, `--ctime`
: Use the changed time. With a long view option, show it instead of the modified time. With `-t`, or without a long view option, sort by it, newest first.

`-u`, `--atime`
: Use the accessed time. With a long view option, show it instead of the modified time. With `-t`, or without a long view option, sort by it, newest first.

Like `ls`, the last of `--sort`, `-t`, `-S`, `-U`, `-X`, `-v`, and `-f` picks the sort order, and the last of `-c`, `-u`, and `--time` picks the time field.

`-I`, `--ignore-glob=GLOBS`, `--ignore=GLOBS`
: Glob patterns, pipe-separated, of files to ignore.
Like `ls -I`, it can be given more than once, and the patterns add up.

`-B`, `--ignore-backups`
: Do not list files whose names end with ‘`~`’.

`--git-ignore` [if eza was built with git support]
: Do not list files that are ignored by Git.

`--group-directories-first`
: List directories before other files.

`--group-directories-last`
: List directories after other files.

`--only-dirs`
: List only directories, not files.

`--only-files`
: List only files, not directories.

`--show-symlinks`
: Explicitly show symbolic links (when used with `--only-files` | `--only-dirs`)

`--no-symlinks`
: Do not show symbolic links

LONG VIEW OPTIONS
=================

These options are available when running with `--long` (`-l`).
`-g`, `-o`, and `-n` also turn on the long view, like `-l` does.
`-s` and `-Z` also work without a long view: like `ls`, they put the block size or the security context before each file name.

`-h`, `--human-readable`
: List file sizes with binary prefixes, where 1K is 1024 bytes. The same as `--binary`.

`--binary`
: List file sizes with binary prefixes.

`--bytes`
: List file sizes in bytes, without any prefixes.

`--changed`
: Use the changed timestamp field.

`--group`
: List each file’s group.

`-G`, `--no-group`
: Do not list each file’s group. This overrides `-g` and `--group`.

`--smart-group`
: Only show group if it has a different name from owner

`--header`
: Add a header row to each column.

`--links`
: List each file’s number of hard links.

`-i`, `--inode`
: List each file’s inode number.

`--loc[=MODE]`
: Add a language column and a lines-of-code column to the long view.

: Only regular files in a recognised programming language are counted; counting is comment-aware, so the code column excludes comment and blank lines.

: Valid modes are ‘`lines`’ (the count of code lines), ‘`percent`’ (each file’s share of the code in the whole tree), and ‘`both`’ (the default). In `percent` and `both` modes the denominator is the total code across the recursed tree, or the git repository if one is present.

`--modified`
: Use the modified timestamp field.

`-M`, `--mounts`
: Show mount details (Linux and Mac only)

`--numeric`
: List numeric user and group IDs.
Unlike `-n`, it does not turn on the long view.

`-O`, `--flags`
: List file flags on Mac and BSD systems and file attributes on Windows systems.  By default, Windows attributes are displayed in a long form.  To display in attributes as single character set the environment variable `EZA_WINDOWS_ATTRIBUTES=short`.  On BSD systems see chflags(1) for a list of file flags and their meanings.

`-s`, `--size`
: List each file’s size of allocated file system blocks. The same as `--blocksize`.
Without a long view, the size is put before each file name, like `ls -s`.

`--blocksize`
: List each file’s size of allocated file system blocks.

`--time=WORD`
: Which timestamp field to list. With `-t`, or without a long view option, also sort by it, newest first.

: Valid timestamp fields are ‘`modified`’, ‘`changed`’, ‘`accessed`’, and ‘`created`’, and the `ls` words ‘`mtime`’ and ‘`modification`’, ‘`ctime`’ and ‘`status`’, ‘`atime`’, ‘`access`’ and ‘`use`’, and ‘`birth`’ and ‘`creation`’.

`--time-style=STYLE`
: How to format timestamps.

: Valid timestamp styles are ‘`default`’, ‘`iso`’, ‘`long-iso`’, ‘`full-iso`’, ‘`relative`’, or a custom style ‘`+<FORMAT>`’ (e.g., ‘`+%Y-%m-%d %H:%M`’ => ‘`2023-09-30 13:00`’).

`<FORMAT>` should be a chrono format string.  For details on the chrono format syntax, please read: https://docs.rs/chrono/latest/chrono/format/strftime/index.html .

Alternatively, `<FORMAT>` can be a two line string, the first line will be used for non-recent files and the second for recent files.  E.g., if `<FORMAT>` is "`%Y-%m-%d %H<newline>--%m-%d %H:%M`", non-recent files => "`2022-12-30 13`", recent files => "`--09-30 13:34`".

`--total-size`
: Show recursive directory size (unix only).

`--accessed`
: Use the accessed timestamp field.

`--created`
: Use the created timestamp field.

`--no-permissions`
: Suppress the permissions field.

`--octal-permissions`
: List each file's permissions in octal format.

`--no-filesize`
: Suppress the file size field.

`--no-user`
: Suppress the user field.

`--no-time`
: Suppress the time field.

`-@`, `--extended`
: List each file’s extended attributes and sizes.

`-Z`, `--context`
: List each file's security context.
Without a long view, the context is put before each file name, like `ls -Z`.

`--git`  [if eza was built with git support]
: List each file’s Git status, if tracked.
This adds a two-character column indicating the staged and unstaged statuses respectively. The status character can be ‘`-`’ for not modified, ‘`M`’ for a modified file, ‘`N`’ for a new file, ‘`D`’ for deleted, ‘`R`’ for renamed, ‘`T`’ for type-change, ‘`I`’ for ignored, and ‘`U`’ for conflicted. Directories will be shown to have the status of their contents, which is how ‘deleted’ is possible if a directory contains a file that has a certain status, it will be shown to have that status.

`--git-repos` [if eza was built with git support]
: List each directory’s Git status, if tracked.
Symbols shown are `|`= clean, `+`= dirty, and `~`= for unknown.

`--git-repos-no-status` [if eza was built with git support]
: List if a directory is a Git repository, but not its status.
All Git repository directories will be shown as (themed) `-` without status indicated.


`--no-git`
: Don't show Git status (always overrides `--git`, `--git-repos`, `--git-repos-no-status`)


UNSUPPORTED LS OPTIONS
======================

These `ls` options are recognised, but eza does not support them. Giving one of them makes eza exit with an error saying the option is not supported, rather than silently ignoring it.

`-T`, `--tabsize=COLS`
: Assume tab stops at each COLS.

`-D`, `--dired`
: Generate output designed for Emacs’ dired mode.

`-H`, `--dereference-command-line`
: Follow symbolic links given on the command line.

`-m`, `--format-commas`
: Fill width with a comma separated list of entries.

`-b`, `--escape`
: Print C-style escapes for nongraphic characters.

`-Q`, `--quote-name`
: Enclose entry names in double quotes.

`-N`, `--literal`
: Print entry names without quoting.

`-q`, `--hide-control-chars`
: Print ‘`?`’ instead of nongraphic characters.

`-p`, `--indicator-slash`
: Append ‘`/`’ to directories.

`-k`, `--kibibytes`
: Use 1024-byte blocks for file system usage.


ENVIRONMENT VARIABLES
=====================

If an environment variable prefixed with `EZA_` is not set, for backward compatibility, it will default to its counterpart starting with `EXA_`.

eza responds to the following environment variables:

## `COLUMNS`

Overrides the width of the terminal, in characters, however, `-w` takes precedence.

For example, ‘`COLUMNS=80 eza`’ will show a grid view with a maximum width of 80 characters.

This option won’t do anything when eza’s output doesn’t wrap, such as when using the `--long` view.

## `EZA_STRICT`

Enables _strict mode_, which will make eza error when two command-line options are incompatible.

Usually, options can override each other going right-to-left on the command line, so that eza can be given aliases: creating an alias ‘`eza=eza --sort=ext`’ then running ‘`eza --sort=size`’ with that alias will run ‘`eza --sort=ext --sort=size`’, and the sorting specified by the user will override the sorting specified by the alias.

In strict mode, the two options will not co-operate, and eza will error.

This option is intended for use with automated scripts and other situations where you want to be certain you’re typing in the right command.

## `EZA_GRID_ROWS`

Limits the grid-details view (‘`eza --grid --long`’) so it’s only activated when at least the given number of rows of output would be generated.

With widescreen displays, it’s possible for the grid to look very wide and sparse, on just one or two lines with none of the columns lining up.
By specifying a minimum number of rows, you can only use the view if it’s going to be worth using.

## `EZA_ICON_SPACING`

Specifies the number of spaces to print between an icon (see the ‘`--icons`’ option) and its file name.

Different terminals display icons differently, as they usually take up more than one character width on screen, so there’s no “standard” number of spaces that eza can use to separate an icon from text. One space may place the icon too close to the text, and two spaces may place it too far away. So the choice is left up to the user to configure depending on their terminal emulator.

## `NO_COLOR`

Disables colours in the output (regardless of its value). Can be overridden by `--color` option.

See `https://no-color.org/` for details.

## `LS_COLORS`, `EZA_COLORS`

Specifies the colour scheme used to highlight files based on their name and kind, as well as highlighting metadata and parts of the UI.

For more information on the format of these environment variables, see the [eza_colors.5.md](eza_colors.5.md) manual page.

## `EZA_OVERRIDE_GIT`

Overrides any `--git` or `--git-repos` argument

## `EZA_MIN_LUMINANCE`
Specifies the minimum luminance to use when color-scale is active. It's value can be between -100 to 100.

## `EZA_ICONS_AUTO`

If set, automates the same behavior as using `--icons` or `--icons=auto`. Useful for if you always want to have icons enabled.

Any explicit use of the `--icons=WHEN` flag overrides this behavior.

## `EZA_STDIN_SEPARATOR`

Specifies the separator to use when file names are read from stdin with `--stdin`. Defaults to newline.

## `EZA_CONFIG_DIR`

Specifies the directory where eza will look for its configuration and theme files. Defaults to `$XDG_CONFIG_HOME/eza` or `$HOME/.config/eza` if `XDG_CONFIG_HOME` is not set.

EXIT STATUSES
=============

0
: If everything goes OK.

1
: If there was an I/O error during operation.

3
: If there was a problem with the command-line arguments.

13
: If permission is denied to access a path.


AUTHOR
======

eza is maintained by Christina Sørensen and many other contributors.

**Source code:** `https://github.com/eza-community/eza` \
**Contributors:** `https://github.com/eza-community/eza/graphs/contributors`

Our infinite thanks to Benjamin ‘ogham’ Sago and all the other contributors of exa, from which eza was forked.

SEE ALSO
========

- [**eza_colors**(5)](eza_colors.5.md)
- [**eza_colors-explanation**(5)](eza_colors-explanation.5.md)
