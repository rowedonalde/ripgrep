# NAME

rg - recursively search current directory for lines matching a pattern

# SYNOPSIS

rg [*options*] -e PATTERN ... [*<*path*> ...*]

rg [*options*] <*pattern*> [*<*path*> ...*]

rg [*options*] --files [*<*path*> ...*]

rg [*options*] --type-list

rg [*options*] --help

rg [*options*] --version

# DESCRIPTION

rg (ripgrep) combines the usability of The Silver Searcher (an ack clone) with
the raw speed of grep.

# COMMON OPTIONS

-a, --text
: Search binary files as if they were text.

-c, --count
: Only show count of line matches for each file.

--color *WHEN*
: Whether to use coloring in match. Valid values are never, always or auto.
  [default: auto]

-e, --regexp *PATTERN* ...
: Use PATTERN to search. This option can be provided multiple times, where all
  patterns given are searched.

-F, --fixed-strings
: Treat the pattern as a literal string instead of a regular expression.

-g, --glob *GLOB* ...
: Include or exclude files for searching that match the given glob. This always
  overrides any other ignore logic. Multiple glob flags may be used. Globbing
  rules match .gitignore globs. Precede a glob with a '!' to exclude it.

-h, --help
: Show this usage message.

-i, --ignore-case
: Case insensitive search. Overridden by --case-sensitive.

-n, --line-number
: Show line numbers (1-based). This is enabled by default at a tty.

-N, --no-line-number
: Suppress line numbers.

-q, --quiet
: Do not print anything to stdout. If a match is found in a file, stop
  searching that file.

-r, --replace *ARG*
: Replace every match with the string given. Capture group indices (e.g., $5)
  and names (e.g., $foo) are supported.

-t, --type *TYPE* ...
: Only search files matching TYPE. Multiple type flags may be provided. Use the
  --type-list flag to list all available types.

-T, --type-not *TYPE* ...
: Do not search files matching TYPE. Multiple not-type flags may be provided.

-u, --unrestricted ...
: Reduce the level of 'smart' searching. A single -u doesn't respect .gitignore
  (etc.) files. Two -u flags will search hidden files and directories. Three
  -u flags will search binary files. -uu is equivalent to grep -r, and -uuu is
  equivalent to grep -a -r.

-v, --invert-match
: Invert matching.

-w, --word-regexp
: Only show matches surrounded by word boundaries. This is equivalent to
  putting \\b before and after the search pattern.

# LESS COMMON OPTIONS

-A, --after-context *NUM*
: Show NUM lines after each match.

-B, --before-context *NUM*
: Show NUM lines before each match.

-C, --context *NUM*
: Show NUM lines before and after each match.

--column
: Show column numbers (1 based) in output. This only shows the column
  numbers for the first match on each line. Note that this doesn't try
  to account for Unicode. One byte is equal to one column.

--context-separator *ARG*
: The string to use when separating non-continuous context lines. Escape
  sequences may be used. [default: --]

--debug
: Show debug messages.

--files
: Print each file that would be searched (but don't search).

-l, --files-with-matches
: Only show path of each file with matches.

-H, --with-filename
: Prefix each match with the file name that contains it. This is the
  default when more than one file is searched.

--no-filename
: Never show the filename for a match. This is the default when
  one file is searched.

--heading
: Show the file name above clusters of matches from each file.
  This is the default mode at a tty.

--no-heading
: Don't show any file name heading.

--hidden
: Search hidden directories and files. (Hidden directories and files are
  skipped by default.)

-L, --follow
: Follow symlinks.

--maxdepth *NUM*
: Descend at most NUM directories below the command line arguments.
  A value of zero searches only the starting-points themselves.

--mmap
: Search using memory maps when possible. This is enabled by default
  when ripgrep thinks it will be faster. (Note that mmap searching
  doesn't currently support the various context related options.)

--no-mmap
: Never use memory maps, even when they might be faster.

--no-ignore
: Don't respect ignore files (.gitignore, .ignore, etc.)
  This implies --no-ignore-parent.

--no-ignore-parent
: Don't respect ignore files in parent directories.

--no-ignore-vcs
: Don't respect version control ignore files (e.g., .gitignore).
  Note that .ignore files will continue to be respected.

--null
: Whenever a file name is printed, follow it with a NUL byte.
  This includes printing filenames before matches, and when printing
  a list of matching files such as with --count, --files-with-matches
  and --files.

-p, --pretty
: Alias for --color=always --heading -n.

-s, --case-sensitive
: Search case sensitively. This overrides --ignore-case and --smart-case.

-S, --smart-case
: Search case insensitively if the pattern is all lowercase.
  Search case sensitively otherwise. This is overridden by either
  --case-sensitive or --ignore-case.

-j, --threads *ARG*
: The number of threads to use. Defaults to the number of logical CPUs
  (capped at 6). [default: 0]

--version
: Show the version number of ripgrep and exit.

--vimgrep
: Show results with every match on its own line, including line
  numbers and column numbers. (With this option, a line with more
  than one match of the regex will be printed more than once.)

# FILE TYPE MANAGEMENT OPTIONS

--type-list
: Show all supported file types and their associated globs.

--type-add *ARG* ...
: Add a new glob for a particular file type. Only one glob can be added
  at a time. Multiple --type-add flags can be provided. Unless --type-clear
  is used, globs are added to any existing globs inside of ripgrep. Note that
  this must be passed to every invocation of rg.

  Example: `--type-add html:*.html`

--type-clear *TYPE* ...
: Clear the file type globs previously defined for TYPE. This only clears
  the default type definitions that are found inside of ripgrep. Note
  that this must be passed to every invocation of rg.
