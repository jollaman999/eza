// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
//! Parsing the options for `FileFilter`.

use clap::ArgMatches;
use clap::parser::ValueSource;

use crate::fs::DotFilter;
use crate::fs::filter::{
    FileFilter, FileFilterFlags, GitIgnore, IgnorePatterns, SortCase, SortField,
};

use crate::options::OptionsError;
use crate::options::parser::{SortArg, TimeArgs};
use crate::options::view::long_given;

/// The options that pick a sort order. The last one given wins.
const SORT_ARGS: &[&str] = &[
    "sort",
    "sort-time",
    "sort-size",
    "unsorted",
    "sort-extension",
    "sort-version",
    "unsorted-all",
];

/// The options that pick which time field to sort by and, in the long view,
/// to show. The last one given wins.
pub(super) const TIME_ARGS: &[&str] = &["time", "ctime", "atime"];

/// Whether the option was given on the command line, rather than taking its
/// default value.
fn given(matches: &ArgMatches, id: &str) -> bool {
    matches.value_source(id) == Some(ValueSource::CommandLine)
}

/// The option out of `ids` that was given last on the command line.
pub(super) fn last_given(matches: &ArgMatches, ids: &[&'static str]) -> Option<&'static str> {
    ids.iter()
        .copied()
        .filter(|id| given(matches, id))
        .max_by_key(|id| matches.indices_of(id).and_then(Iterator::max))
}

/// In strict mode, giving two different options out of `ids` is an error.
/// Giving the same one more than once is not.
fn strict_check_group(matches: &ArgMatches, ids: &[&'static str]) -> Result<(), OptionsError> {
    let mut given_ids = ids.iter().copied().filter(|id| given(matches, id));
    match (given_ids.next(), given_ids.next()) {
        (Some(a), Some(b)) => Err(OptionsError::Conflict(a, b)),
        _ => Ok(()),
    }
}

impl FileFilter {
    /// Determines which of all the file filter options to use.
    pub fn deduce(matches: &ArgMatches, strict: bool) -> Result<Self, OptionsError> {
        use FileFilterFlags as FFF;
        let mut filter_flags: Vec<FileFilterFlags> = vec![];

        for (flag, filter_flag) in &[
            ("reverse", FFF::Reverse),
            ("only-dirs", FFF::OnlyDirs),
            ("only-files", FFF::OnlyFiles),
            ("no-symlinks", FFF::NoSymlinks),
            ("show-symlinks", FFF::ShowSymlinks),
            ("dirs-last", FFF::ListDirsLast),
            ("dirs-first", FFF::ListDirsFirst),
        ] {
            if matches.get_flag(flag) {
                filter_flags.push(filter_flag.clone());
            }
        }

        Ok(Self {
            no_symlinks: matches.get_flag("no-symlinks"),
            show_symlinks: matches.get_flag("show-symlinks"),
            flags: filter_flags,
            sort_field: SortField::deduce(matches, strict)?,
            dot_filter: DotFilter::deduce(matches, strict)?,
            ignore_patterns: IgnorePatterns::deduce(matches)?,
            git_ignore: GitIgnore::deduce(matches),
        })
    }
}

impl SortField {
    /// Determines which sort field to use like `ls` does: out of `--sort`,
    /// `-t`, `-S`, `-U`, `-X`, `-v` and `-f`, the last one given wins.
    ///
    /// Sorting by time picks the time field from the last of `-c`, `-u` and
    /// `--time`, newest first. Giving one of those without a sort option
    /// sorts by that time too, unless the long view shows it instead.
    ///
    /// Strict mode rejects two different sort options, or two different
    /// time options.
    pub fn deduce(matches: &ArgMatches, strict: bool) -> Result<Self, OptionsError> {
        if strict {
            strict_check_group(matches, SORT_ARGS)?;
            strict_check_group(matches, TIME_ARGS)?;
        }

        let field = match last_given(matches, SORT_ARGS) {
            Some("sort-time") => Self::newest_first(matches),
            Some("sort-size") => Self::Size,
            Some("unsorted" | "unsorted-all") => Self::Unsorted,
            Some("sort-extension") => Self::Extension(SortCase::AaBbCc),
            Some("sort-version") => Self::default(),
            Some(_) => match matches.get_one::<SortArg>("sort") {
                Some(SortArg::Field(field)) => *field,
                Some(SortArg::Time) => Self::newest_first(matches),
                None => Self::default(),
            },
            None if last_given(matches, TIME_ARGS).is_some() && !long_given(matches) => {
                Self::newest_first(matches)
            }
            None => Self::default(),
        };

        Ok(field)
    }

    /// The newest-first sort field for the time field picked by the last of
    /// `-c`, `-u` and `--time`, which is the modified time by default.
    fn newest_first(matches: &ArgMatches) -> Self {
        match last_given(matches, TIME_ARGS) {
            Some("ctime") => Self::ChangedAge,
            Some("atime") => Self::AccessedAge,
            Some(_) => match matches.get_one::<TimeArgs>("time") {
                Some(TimeArgs::Changed) => Self::ChangedAge,
                Some(TimeArgs::Accessed) => Self::AccessedAge,
                Some(TimeArgs::Created) => Self::CreatedAge,
                Some(TimeArgs::Modified) | None => Self::ModifiedAge,
            },
            None => Self::ModifiedAge,
        }
    }
}

// I’ve gone back and forth between whether to sort case-sensitively or
// insensitively by default. The default string sort in most programming
// languages takes each character’s ASCII value into account, sorting
// “Documents” before “apps”, but there’s usually an option to ignore
// characters’ case, putting “apps” before “Documents”.
//
// The argument for following case is that it’s easy to forget whether an item
// begins with an uppercase or lowercase letter and end up having to scan both
// the uppercase and lowercase sub-lists to find the item you want. If you
// happen to pick the sublist it’s not in, it looks like it’s missing, which
// is worse than if you just take longer to find it.
// (https://ux.stackexchange.com/a/79266)
//
// The argument for ignoring case is that it makes exa sort files differently
// from shells. A user would expect a directory’s files to be in the same
// order if they used “exa ~/directory” or “exa ~/directory/*”, but exa sorts
// them in the first case, and the shell in the second case, so they wouldn’t
// be exactly the same if exa does something non-conventional.
//
// However, exa already sorts files differently: it uses natural sorting from
// the natord crate, sorting the string “2” before “10” because the number’s
// smaller, because that’s usually what the user expects to happen. Users will
// name their files with numbers expecting them to be treated like numbers,
// rather than lists of numeric characters.
//
// In the same way, users will name their files with letters expecting the
// order of the letters to matter, rather than each letter’s character’s ASCII
// value. So exa breaks from tradition and ignores case while sorting:
// “apps” first, then “Documents”.
//
// You can get the old behaviour back by sorting with `--sort=Name`.
impl Default for SortField {
    fn default() -> Self {
        Self::Name(SortCase::AaBbCc)
    }
}

impl DotFilter {
    /// Determines the dot filter the way `ls` does: `--all` and
    /// `--unsorted-all` show dotfiles together with `.` and `..`, and
    /// `--almost-all` shows dotfiles only. Giving `--all` more than once is
    /// the same as giving it once.
    ///
    /// When `--almost-all` and either of the others are given, the one that
    /// comes last on the command line wins, and strict mode rejects the
    /// combination.
    ///
    /// It also checks for the `--tree` option, because listing the parent
    /// directory in tree mode would loop onto itself, so `--all` only shows
    /// dotfiles there.
    pub fn deduce(matches: &ArgMatches, strict: bool) -> Result<Self, OptionsError> {
        let has_all = matches.get_count("all") > 0 || matches.get_flag("unsorted-all");
        let has_almost_all = matches.get_flag("almost-all");

        let all_wins = match (has_all, has_almost_all) {
            (false, false) => return Ok(Self::JustFiles),
            (true, false) => true,
            (false, true) => false,
            (true, true) => {
                if strict {
                    let all = if matches.get_count("all") > 0 {
                        "all"
                    } else {
                        "unsorted-all"
                    };
                    return Err(OptionsError::Conflict(all, "almost-all"));
                }
                last_given(matches, &["all", "unsorted-all", "almost-all"]) != Some("almost-all")
            }
        };

        if all_wins && !matches.get_flag("tree") {
            Ok(Self::DotfilesAndDots)
        } else {
            Ok(Self::Dotfiles)
        }
    }
}

impl IgnorePatterns {
    /// Determines the set of glob patterns to use based on the
    /// `--ignore-glob` argument’s value. This is a list of strings
    /// separated by pipe (`|`) characters, given in any order.
    pub fn deduce(matches: &ArgMatches) -> Result<Self, OptionsError> {
        // If there are no inputs, we return a set of patterns that doesn’t
        // match anything, rather than, say, `None`.
        let Some(inputs) = matches.get_one::<String>("ignore-glob") else {
            return Ok(Self::empty());
        };

        // Awkwardly, though, a glob pattern can be invalid, and we need to
        // deal with invalid patterns somehow.
        let (patterns, mut errors) = Self::parse_from_iter(inputs.split('|'));

        // It can actually return more than one glob error,
        // but we only use one. (TODO)
        match errors.pop() {
            Some(e) => Err(e.into()),
            None => Ok(patterns),
        }
    }
}

impl GitIgnore {
    pub fn deduce(matches: &ArgMatches) -> Self {
        if matches.get_flag("git-ignore") {
            Self::CheckAndIgnore
        } else {
            Self::Off
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::*;
    use crate::options::parser::test::{mock_cli, mock_cli_try};

    #[test]
    fn deduce_git_ignore_off() {
        assert_eq!(GitIgnore::deduce(&mock_cli(vec![""])), GitIgnore::Off);
    }

    #[test]
    fn deduce_git_ignore_on() {
        assert_eq!(
            GitIgnore::deduce(&mock_cli(vec!["--git-ignore"])),
            GitIgnore::CheckAndIgnore
        );
    }

    #[test]
    fn deduce_ignore_patterns_empty() {
        assert_eq!(
            IgnorePatterns::deduce(&mock_cli(vec![""])),
            Ok(IgnorePatterns::empty())
        );
    }

    #[test]
    fn deduce_ignore_patterns_one() {
        let pattern = OsString::from("*.o");
        let (res, _) = IgnorePatterns::parse_from_iter(pattern.to_string_lossy().split('|'));

        assert_eq!(
            IgnorePatterns::deduce(&mock_cli(vec!["--ignore-glob", "*.o"])),
            Ok(res)
        );
    }

    #[test]
    fn deduce_ignore_patterns_error() {
        let pattern = OsString::from("[");
        let (_, mut e) = IgnorePatterns::parse_from_iter(pattern.to_string_lossy().split('|'));
        assert_eq!(
            IgnorePatterns::deduce(&mock_cli(vec!["--ignore-glob", "["])),
            Err(e.pop().unwrap().into())
        );
    }

    #[test]
    fn deduce_dot_filter_just_files() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec![""]), false),
            Ok(DotFilter::JustFiles)
        );
    }

    #[test]
    fn deduce_dot_filter_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-a"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--all"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
    }

    #[test]
    fn deduce_dot_filter_all_twice() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-aa"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--all", "--all"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
    }

    #[test]
    fn deduce_dot_filter_all_thrice() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-aaa"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
    }

    #[test]
    fn deduce_dot_filter_all_thrice_strict() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-aaa"]), true),
            Ok(DotFilter::DotfilesAndDots)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--all", "--all", "--all"]), true),
            Ok(DotFilter::DotfilesAndDots)
        );
    }

    #[test]
    fn deduce_dot_filter_tree_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--tree", "-a"]), false),
            Ok(DotFilter::Dotfiles)
        );
    }

    #[test]
    fn deduce_dot_filter_tree_all_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--tree", "-aa"]), false),
            Ok(DotFilter::Dotfiles)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--all", "--all", "--tree"]), true),
            Ok(DotFilter::Dotfiles)
        );
    }

    #[test]
    fn deduce_dot_filter_almost_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-A"]), false),
            Ok(DotFilter::Dotfiles)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--almost-all"]), false),
            Ok(DotFilter::Dotfiles)
        );
    }

    #[test]
    fn deduce_dot_filter_tree_almost_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--tree", "-A"]), false),
            Ok(DotFilter::Dotfiles)
        );
    }

    #[test]
    fn deduce_dot_filter_all_then_almost_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-a", "-A"]), false),
            Ok(DotFilter::Dotfiles)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-aa", "-A"]), false),
            Ok(DotFilter::Dotfiles)
        );
    }

    #[test]
    fn deduce_dot_filter_almost_all_then_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-A", "-a"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-a", "-A", "-a"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-A", "-a", "-A"]), false),
            Ok(DotFilter::Dotfiles)
        );
    }

    #[test]
    fn deduce_dot_filter_all_almost_all_strict() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-a", "-A"]), true),
            Err(OptionsError::Conflict("all", "almost-all"))
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-A", "-a"]), true),
            Err(OptionsError::Conflict("all", "almost-all"))
        );
    }

    #[test]
    fn deduce_dot_filter_unsorted_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-f"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--unsorted-all"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-ff"]), true),
            Ok(DotFilter::DotfilesAndDots)
        );
    }

    #[test]
    fn deduce_dot_filter_tree_unsorted_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["--tree", "-f"]), false),
            Ok(DotFilter::Dotfiles)
        );
    }

    #[test]
    fn deduce_dot_filter_unsorted_all_and_almost_all() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-f", "-A"]), false),
            Ok(DotFilter::Dotfiles)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-A", "-f"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-a", "-A", "-f"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-f", "-A", "-a"]), false),
            Ok(DotFilter::DotfilesAndDots)
        );
    }

    #[test]
    fn deduce_dot_filter_unsorted_all_strict() {
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-a", "-f"]), true),
            Ok(DotFilter::DotfilesAndDots)
        );
        assert_eq!(
            DotFilter::deduce(&mock_cli(vec!["-A", "-f"]), true),
            Err(OptionsError::Conflict("unsorted-all", "almost-all"))
        );
    }

    fn sort_field(args: Vec<&str>) -> Result<SortField, OptionsError> {
        SortField::deduce(&mock_cli(args), false)
    }

    #[test]
    fn deduce_sort_default() {
        assert_eq!(sort_field(vec![""]), Ok(SortField::default()));
        assert_eq!(sort_field(vec!["-l"]), Ok(SortField::default()));
    }

    #[test]
    fn deduce_sort_flags() {
        assert_eq!(sort_field(vec!["-t"]), Ok(SortField::ModifiedAge));
        assert_eq!(sort_field(vec!["--sort-time"]), Ok(SortField::ModifiedAge));
        assert_eq!(sort_field(vec!["-S"]), Ok(SortField::Size));
        assert_eq!(sort_field(vec!["--sort-size"]), Ok(SortField::Size));
        assert_eq!(sort_field(vec!["-U"]), Ok(SortField::Unsorted));
        assert_eq!(sort_field(vec!["--unsorted"]), Ok(SortField::Unsorted));
        assert_eq!(
            sort_field(vec!["-X"]),
            Ok(SortField::Extension(SortCase::AaBbCc))
        );
        assert_eq!(
            sort_field(vec!["--sort-extension"]),
            Ok(SortField::Extension(SortCase::AaBbCc))
        );
        assert_eq!(sort_field(vec!["-v"]), Ok(SortField::default()));
        assert_eq!(sort_field(vec!["--sort-version"]), Ok(SortField::default()));
        assert_eq!(sort_field(vec!["-f"]), Ok(SortField::Unsorted));
        assert_eq!(sort_field(vec!["--unsorted-all"]), Ok(SortField::Unsorted));
    }

    #[test]
    fn deduce_sort_words() {
        assert_eq!(sort_field(vec!["--sort=size"]), Ok(SortField::Size));
        assert_eq!(sort_field(vec!["--sort=time"]), Ok(SortField::ModifiedAge));
        assert_eq!(sort_field(vec!["--sort=none"]), Ok(SortField::Unsorted));
        assert_eq!(sort_field(vec!["--sort=version"]), Ok(SortField::default()));
        assert_eq!(
            sort_field(vec!["--sort=extension"]),
            Ok(SortField::Extension(SortCase::AaBbCc))
        );
        assert_eq!(sort_field(vec!["--sort=date"]), Ok(SortField::ModifiedDate));
        assert_eq!(sort_field(vec!["--sort=age"]), Ok(SortField::ModifiedAge));
        assert_eq!(
            sort_field(vec!["-u", "--sort=time"]),
            Ok(SortField::AccessedAge)
        );
        assert_eq!(
            sort_field(vec!["-l", "-c", "--sort=time"]),
            Ok(SortField::ChangedAge)
        );
    }

    #[test]
    fn deduce_sort_time_field() {
        assert_eq!(sort_field(vec!["-tc"]), Ok(SortField::ChangedAge));
        assert_eq!(sort_field(vec!["-t", "--ctime"]), Ok(SortField::ChangedAge));
        assert_eq!(sort_field(vec!["-tu"]), Ok(SortField::AccessedAge));
        assert_eq!(
            sort_field(vec!["-t", "--atime"]),
            Ok(SortField::AccessedAge)
        );
        assert_eq!(sort_field(vec!["-ltc"]), Ok(SortField::ChangedAge));
        assert_eq!(sort_field(vec!["-ltu"]), Ok(SortField::AccessedAge));
        for (word, field) in [
            ("atime", SortField::AccessedAge),
            ("access", SortField::AccessedAge),
            ("use", SortField::AccessedAge),
            ("accessed", SortField::AccessedAge),
            ("ctime", SortField::ChangedAge),
            ("status", SortField::ChangedAge),
            ("changed", SortField::ChangedAge),
            ("birth", SortField::CreatedAge),
            ("creation", SortField::CreatedAge),
            ("created", SortField::CreatedAge),
            ("mtime", SortField::ModifiedAge),
            ("modification", SortField::ModifiedAge),
            ("modified", SortField::ModifiedAge),
        ] {
            assert_eq!(sort_field(vec!["-t", "--time", word]), Ok(field), "{word}");
            assert_eq!(
                sort_field(vec!["-l", "-t", "--time", word]),
                Ok(field),
                "{word}"
            );
        }
    }

    #[test]
    fn deduce_sort_time_field_last_wins() {
        assert_eq!(
            sort_field(vec!["-t", "-c", "-u"]),
            Ok(SortField::AccessedAge)
        );
        assert_eq!(
            sort_field(vec!["-t", "-u", "-c"]),
            Ok(SortField::ChangedAge)
        );
        assert_eq!(
            sort_field(vec!["-t", "-u", "--time=birth"]),
            Ok(SortField::CreatedAge)
        );
        assert_eq!(
            sort_field(vec!["-t", "--time=birth", "-u"]),
            Ok(SortField::AccessedAge)
        );
    }

    #[test]
    fn deduce_sort_time_without_sort_option() {
        assert_eq!(sort_field(vec!["-c"]), Ok(SortField::ChangedAge));
        assert_eq!(sort_field(vec!["-u"]), Ok(SortField::AccessedAge));
        assert_eq!(sort_field(vec!["--time=atime"]), Ok(SortField::AccessedAge));
        assert_eq!(sort_field(vec!["-1", "-u"]), Ok(SortField::AccessedAge));
        assert_eq!(sort_field(vec!["-lc"]), Ok(SortField::default()));
        assert_eq!(sort_field(vec!["-lu"]), Ok(SortField::default()));
        for args in [vec!["-gc"], vec!["-oc"], vec!["-nc"], vec!["-u", "-g"]] {
            assert_eq!(
                sort_field(args.clone()),
                Ok(SortField::default()),
                "{args:?}"
            );
        }
        assert_eq!(
            sort_field(vec!["-l", "--time=atime"]),
            Ok(SortField::default())
        );
        assert_eq!(sort_field(vec!["-u", "-S"]), Ok(SortField::Size));
        assert_eq!(
            sort_field(vec!["-u", "--sort=name"]),
            Ok(SortField::default())
        );
    }

    #[test]
    fn deduce_sort_last_wins() {
        assert_eq!(sort_field(vec!["-St"]), Ok(SortField::ModifiedAge));
        assert_eq!(sort_field(vec!["-tS"]), Ok(SortField::Size));
        assert_eq!(
            sort_field(vec!["-t", "--sort=name"]),
            Ok(SortField::default())
        );
        assert_eq!(
            sort_field(vec!["--sort=name", "-t"]),
            Ok(SortField::ModifiedAge)
        );
        assert_eq!(
            sort_field(vec!["-t", "-S", "-t"]),
            Ok(SortField::ModifiedAge)
        );
        assert_eq!(sort_field(vec!["-U", "-v"]), Ok(SortField::default()));
        assert_eq!(sort_field(vec!["-X", "-f"]), Ok(SortField::Unsorted));
        assert_eq!(
            sort_field(vec!["-f", "-X"]),
            Ok(SortField::Extension(SortCase::AaBbCc))
        );
        assert_eq!(
            sort_field(vec!["--sort=size", "--sort=time"]),
            Ok(SortField::ModifiedAge)
        );
    }

    #[test]
    fn deduce_sort_strict() {
        let strict = |args: Vec<&str>| SortField::deduce(&mock_cli(args), true);
        assert_eq!(strict(vec!["-tt"]), Ok(SortField::ModifiedAge));
        assert_eq!(strict(vec!["-cc", "-t"]), Ok(SortField::ChangedAge));
        assert_eq!(strict(vec!["-t", "-u"]), Ok(SortField::AccessedAge));
        assert_eq!(
            strict(vec!["-t", "-S"]),
            Err(OptionsError::Conflict("sort-time", "sort-size"))
        );
        assert_eq!(
            strict(vec!["-t", "--sort=name"]),
            Err(OptionsError::Conflict("sort", "sort-time"))
        );
        assert_eq!(
            strict(vec!["-c", "-u"]),
            Err(OptionsError::Conflict("ctime", "atime"))
        );
        assert_eq!(
            strict(vec!["-u", "--time=atime"]),
            Err(OptionsError::Conflict("time", "atime"))
        );
    }

    #[test]
    fn deduce_sort_field_default() {
        assert_eq!(
            mock_cli(vec![""]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::default()))
        );
    }

    #[test]
    fn deduce_sort_field_name() {
        assert_eq!(
            mock_cli(vec!["--sort", "name"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::Name(SortCase::AaBbCc)))
        );
    }

    #[test]
    fn deduce_sort_field_name_case() {
        assert_eq!(
            mock_cli(vec!["--sort", "Name"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::Name(SortCase::ABCabc)))
        );
    }

    #[test]
    fn deduce_sort_field_name_mix_hidden() {
        assert_eq!(
            mock_cli(vec!["--sort", ".name"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::NameMixHidden(SortCase::AaBbCc)))
        );
    }

    #[test]
    fn deduce_sort_field_name_mix_hidden_case() {
        assert_eq!(
            mock_cli(vec!["--sort", ".Name"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::NameMixHidden(SortCase::ABCabc)))
        );
    }

    #[test]
    fn deduce_sort_field_size() {
        assert_eq!(
            mock_cli(vec!["--sort", "size"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::Size))
        );
    }

    #[test]
    fn deduce_sort_field_extension() {
        assert_eq!(
            mock_cli(vec!["--sort", "ext"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::Extension(SortCase::AaBbCc)))
        );
    }

    #[test]
    fn deduce_sort_field_extension_case() {
        assert_eq!(
            mock_cli(vec!["--sort", "Ext"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::Extension(SortCase::ABCabc)))
        );
    }

    #[test]
    fn deduce_sort_field_date() {
        assert_eq!(
            mock_cli(vec!["--sort", "date"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::ModifiedDate))
        );
    }

    #[test]
    fn deduce_sort_field_time() {
        assert_eq!(
            mock_cli(vec!["--sort", "time"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Time)
        );
    }

    #[test]
    fn deduce_sort_field_mod() {
        assert_eq!(
            mock_cli(vec!["--sort", "mod"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::ModifiedDate))
        );
    }

    #[test]
    fn deduce_sort_field_version() {
        assert_eq!(
            mock_cli(vec!["--sort", "version"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::Name(SortCase::AaBbCc)))
        );
    }

    #[test]
    fn deduce_sort_field_extension_word() {
        assert_eq!(
            mock_cli(vec!["--sort", "extension"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::Extension(SortCase::AaBbCc)))
        );
    }

    #[test]
    fn deduce_sort_field_width_err() {
        assert!(mock_cli_try(vec!["--sort", "width"]).is_err());
    }

    #[test]
    fn deduce_sort_field_age() {
        assert_eq!(
            mock_cli(vec!["--sort", "age"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::ModifiedAge))
        );
    }

    #[test]
    fn deduce_sort_field_old() {
        assert_eq!(
            mock_cli(vec!["--sort", "old"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::ModifiedAge))
        );
    }

    #[test]
    fn deduce_sort_field_ch() {
        assert_eq!(
            mock_cli(vec!["--sort", "ch"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::ChangedDate))
        );
    }

    #[test]
    fn deduce_sort_field_acc() {
        assert_eq!(
            mock_cli(vec!["--sort", "acc"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::AccessedDate))
        );
    }

    #[test]
    fn deduce_sort_field_cr() {
        assert_eq!(
            mock_cli(vec!["--sort", "cr"]).get_one::<SortArg>("sort"),
            Some(&SortArg::Field(SortField::CreatedDate))
        );
    }

    #[test]
    fn deduce_sort_field_err() {
        assert!(mock_cli_try(vec!["--sort", "foo"]).is_err());
    }

    #[test]
    fn deduce_file_filter_default() {
        assert_eq!(
            FileFilter::deduce(&mock_cli(vec![""]), false),
            Ok(FileFilter {
                flags: vec![],
                sort_field: SortField::default(),
                dot_filter: DotFilter::JustFiles,
                ignore_patterns: IgnorePatterns::empty(),
                git_ignore: GitIgnore::Off,
                no_symlinks: false,
                show_symlinks: false,
            })
        );
    }

    #[test]
    fn deduce_file_filter_reverse() {
        assert_eq!(
            FileFilter::deduce(&mock_cli(vec!["--reverse"]), false),
            Ok(FileFilter {
                flags: vec![FileFilterFlags::Reverse],
                sort_field: SortField::default(),
                dot_filter: DotFilter::JustFiles,
                ignore_patterns: IgnorePatterns::empty(),
                git_ignore: GitIgnore::Off,
                no_symlinks: false,
                show_symlinks: false,
            })
        );
    }

    #[test]
    fn deduce_file_filter_only_dirs() {
        assert_eq!(
            FileFilter::deduce(&mock_cli(vec!["--only-dirs"]), false),
            Ok(FileFilter {
                flags: vec![FileFilterFlags::OnlyDirs],
                sort_field: SortField::default(),
                dot_filter: DotFilter::JustFiles,
                ignore_patterns: IgnorePatterns::empty(),
                git_ignore: GitIgnore::Off,
                no_symlinks: false,
                show_symlinks: false,
            })
        );
    }

    #[test]
    fn deduce_file_filter_only_files() {
        assert_eq!(
            FileFilter::deduce(&mock_cli(vec!["--only-files"]), false),
            Ok(FileFilter {
                flags: vec![FileFilterFlags::OnlyFiles],
                sort_field: SortField::default(),
                dot_filter: DotFilter::JustFiles,
                ignore_patterns: IgnorePatterns::empty(),
                git_ignore: GitIgnore::Off,
                no_symlinks: false,
                show_symlinks: false,
            })
        );
    }

    #[test]
    fn deduce_file_filter_unsorted_all() {
        assert_eq!(
            FileFilter::deduce(&mock_cli(vec!["-f"]), false),
            Ok(FileFilter {
                flags: vec![],
                sort_field: SortField::Unsorted,
                dot_filter: DotFilter::DotfilesAndDots,
                ignore_patterns: IgnorePatterns::empty(),
                git_ignore: GitIgnore::Off,
                no_symlinks: false,
                show_symlinks: false,
            })
        );
    }
}
