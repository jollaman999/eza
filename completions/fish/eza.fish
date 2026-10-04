# Meta-stuff
complete -c eza -l version -d "Show version of eza"
complete -c eza -s '?' -l help -d "Show list of command-line options"
complete -c eza -l stdin -d "Read file names from stdin"

# Display options
complete -c eza -s 1 -l oneline -d "Display one entry per line"
complete -c eza -s l -l long -d "Display extended file metadata as a table"
complete -c eza -s g -l long-no-owner -d "Like -l, but list the group instead of the owner"
complete -c eza -s o -l long-no-group -d "Like -l, but do not list the group"
complete -c eza -l grid -d "Display entries in a grid"
complete -c eza -s C -l format-columns -d "Display entries as a grid, listed down columns"
complete -c eza -s x -l across -d "Display entries as a grid, listed across rows"
complete -c eza -s R -l recurse -l recursive -d "Recurse into directories"
complete -c eza -l tree -d "Recurse into directories as a tree"
complete -c eza -s L -l dereference -d "Dereference symbolic links when displaying file information"
complete -c eza -s F -d "Display type indicator by file names (same as --classify=always)"
complete -c eza -l classify -d "Display type indicator by file names" -x -a "
  always\t'Always display type indicators'
  auto\t'Display type indicators if standard output is a terminal'
  automatic\t'Display type indicators if standard output is a terminal'
  never\t'Never display type indicators'
"
complete -c eza -l color \
    -l colour -d "When to use terminal colours" -x -a "
    always\t'Always use colour'
    auto\t'Use colour if standard output is a terminal'
    automatic\t'Use colour if standard output is a terminal'
    never\t'Never use colour'
"
complete -c eza -l color-scale -d "Highlight levels 'field' distinctly" -x -a "
    all\t''
    age\t''
    size\t''
"
complete -c eza -l color-scale-mode \
    -d "Use gradient or fixed colors in --color-scale" -x -a "
    fixed\t'Highlight based on fixed colors'
    gradient\t'Highlight based \'field\' in relation to other files'
"
complete -c eza -l icons -d "When to display icons" -x -a "
  always\t'Always display icons'
  auto\t'Display icons if standard output is a terminal'
  automatic\t'Display icons if standard output is a terminal'
  never\t'Never display icons'
"
complete -c eza -l no-quotes -d "Don't quote file names with spaces"
complete -c eza -l short-nix -d "Abbreviate Nix store hashes in file names and paths"
complete -c eza -l hyperlink -d "When to display entries as hyperlinks" -x -a "
  always\t'Always display entries as hyperlinks'
  auto\t'Display hyperlinks if standard output is a terminal'
  automatic\t'Display hyperlinks if standard output is a terminal'
  never\t'Never display entries as hyperlinks'
"
complete -c eza -l follow-symlinks -d "Drill down into symbolic links that point to directories"
complete -c eza -l absolute -d "Display entries with their absolute path" -x -a "
  on\t'Show absolute path for listed entries'
  follow\t'Show absolute path with followed symlinks'
  off\t'Do not show the absolute path'
"
complete -c eza -l smart-group -d "Only show group if it has a different name from owner"

# Filtering and sorting options
complete -c eza -l group-directories-first -d "Sort directories before other files"
complete -c eza -l group-directories-last -d "Sort directories after other files"
complete -c eza -l git-ignore -d "Ignore files mentioned in '.gitignore'"
complete -c eza -s a -l all -d "Show hidden and 'dot' files, and the '.' and '..' directories"
complete -c eza -s A -l almost-all -d "Show hidden and 'dot' files, but not '.' and '..'"
complete -c eza -s f -l unsorted-all -d "Like -a, and do not sort"
complete -c eza -s d -l treat-dirs-as-files -l directory -d "List directories like regular files"
complete -c eza -l level -d "Limit the depth of recursion" -x -a "1 2 3 4 5 6 7 8 9"
complete -c eza -l code -d "Summarise lines of code by language" -x -a "lines percent both"
complete -c eza -s w -l width -d "Set screen width in columns, 0 means no limit" -x
complete -c eza -s r -l reverse -d "Reverse the sort order"
complete -c eza -l sort -d "Which field to sort by" -x -a "
    accessed\t'Sort by file accessed time'
    age\t'Sort by file modified time (newest first)'
    changed\t'Sort by changed time'
    created\t'Sort by file modified time'
    date\t'Sort by file modified time'
    ext\t'Sort by file extension'
    Ext\t'Sort by file extension (uppercase first)'
    extension\t'Sort by file extension'
    Extension\t'Sort by file extension (uppercase first)'
    filename\t'Sort by filename'
    Filename\t'Sort by filename (uppercase first)'
    inode\t'Sort by file inode'
    modified\t'Sort by file modified time'
    name\t'Sort by filename'
    Name\t'Sort by filename (uppercase first)'
    .name\t'Sort by filename, ignoring the leading dot of hidden files'
    .Name\t'Sort by filename, ignoring the leading dot of hidden files (uppercase first)'
    newest\t'Sort by file modified time (newest first)'
    none\t'Do not sort files at all'
    oldest\t'Sort by file modified time'
    size\t'Sort by file size (largest first)'
    time\t'Sort by time (newest first), the field picked by --time, -c or -u'
    type\t'Sort by file type'
    version\t'Sort by filename, with numbers in natural order'
"
complete -c eza -s t -l sort-time -d "Sort by time, newest first"
complete -c eza -s S -l sort-size -d "Sort by file size, largest first"
complete -c eza -s U -l unsorted -d "Do not sort; list entries in directory order"
complete -c eza -s X -l sort-extension -d "Sort alphabetically by entry extension"
complete -c eza -s v -l sort-version -d "Natural sort of (version) numbers within names"
complete -c eza -s c -l ctime -d "Use the changed time: show it with -l, sort by it with -t or without -l"
complete -c eza -s u -l atime -d "Use the accessed time: show it with -l, sort by it with -t or without -l"

complete -c eza -s I -l ignore-glob -l ignore -d "Ignore files that match these glob patterns" -r
complete -c eza -s B -l ignore-backups -d "Do not list entries ending with ~"
complete -c eza -l only-dirs -d "List only directories"
complete -c eza -l only-files -d "List only files"
complete -c eza -l show-symlinks -d "Explicitly show symbolic links (For use with --only-dirs | --only-files)"
complete -c eza -l no-symlinks -d "Do not show symbolic links"

# Long view options
complete -c eza -s h -l human-readable -d "List file sizes with binary prefixes (same as --binary)"
complete -c eza -l binary -d "List file sizes with binary prefixes"
complete -c eza -l bytes -d "List file sizes in bytes, without any prefixes"
complete -c eza -l group -d "List each file's group"
complete -c eza -s G -l no-group -d "Do not list the group (overrides -g, --group)"
complete -c eza -l header -d "Add a header row to each column"
complete -c eza -l links -d "List each file's number of hard links"
complete -c eza -s i -l inode -d "List each file's inode number"
complete -c eza -l loc -d "Add lines-of-code and language columns" -x -a "lines percent both"
complete -c eza -s s -l size -d "List each file's size of allocated file system blocks (same as --blocksize)"
complete -c eza -l blocksize -d "List each file's size of allocated file system blocks"
complete -c eza -l time -d "Which timestamp field to show and sort by" -x -a "
    modified\t'Display modified time'
    mtime\t'Display modified time'
    changed\t'Display changed time'
    ctime\t'Display changed time'
    accessed\t'Display accessed time'
    atime\t'Display accessed time'
    created\t'Display created time'
    birth\t'Display created time'
"
complete -c eza -l modified -d "Use the modified timestamp field"
complete -c eza -s n -l numeric-uid-gid -d "Like -l, but list numeric user and group IDs"
complete -c eza -l numeric -d "List numeric user and group IDs."
complete -c eza -l changed -d "Use the changed timestamp field"
complete -c eza -l accessed -d "Use the accessed timestamp field"
complete -c eza -l created -d "Use the created timestamp field"
complete -c eza -l time-style -d "How to format timestamps" -x -a "
    default\t'Use the default time style'
    iso\t'Display brief ISO timestamps'
    long-iso\t'Display longer ISO timestamps, up to the minute'
    full-iso\t'Display full ISO timestamps, up to the nanosecond'
    relative\t'Display relative timestamps'
    +FORMAT\t'Use custom time style'
"
complete -c eza -l total-size -d "Show recursive directory size (unix only)"
complete -c eza -l no-permissions -d "Suppress the permissions field"
complete -c eza -l octal-permissions -d "List each file's permission in octal format"
complete -c eza -l no-filesize -d "Suppress the filesize field"
complete -c eza -l no-user -d "Suppress the user field"
complete -c eza -l no-time -d "Suppress the time field"
complete -c eza -s M -l mounts -d "Show mount details"
complete -c eza -s O -l flags -d "List file flags (Mac, BSD, and Windows only)"

# Optional extras
complete -c eza -l git -d "List each file's Git status, if tracked"
complete -c eza -l no-git -d "Suppress Git status"
complete -c eza -l git-repos -d "List each git-repos status and branch name"
complete -c eza -l git-repos-no-status -d "List each git-repos branch name (much faster)"
complete -c eza -s '@' -l extended -d "List each file's extended attributes and sizes"
complete -c eza -s Z -l context -d "List each file's security context"
