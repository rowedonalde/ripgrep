/*!
This module contains *integration* tests. Their purpose is to test the CLI
interface. Namely, that passing a flag does what it says on the tin.

Tests for more fine grained behavior (like the search or the globber) should be
unit tests in their respective modules.
*/

#![allow(dead_code, unused_imports)]

use std::process::Command;

use workdir::WorkDir;

mod hay;
mod workdir;

macro_rules! sherlock {
    ($name:ident, $fun:expr) => {
        sherlock!($name, "Sherlock", $fun);
    };
    ($name:ident, $query:expr, $fun:expr) => {
        sherlock!($name, $query, "sherlock", $fun);
    };
    ($name:ident, $query:expr, $path:expr, $fun:expr) => {
        #[test]
        fn $name() {
            let wd = WorkDir::new(stringify!($name));
            wd.create("sherlock", hay::SHERLOCK);
            let mut cmd = wd.command();
            cmd.arg($query).arg($path);
            $fun(wd, cmd);
        }
    };
}

macro_rules! clean {
    ($name:ident, $query:expr, $path:expr, $fun:expr) => {
        #[test]
        fn $name() {
            let wd = WorkDir::new(stringify!($name));
            let mut cmd = wd.command();
            cmd.arg($query).arg($path);
            $fun(wd, cmd);
        }
    };
}

fn path(unix: &str) -> String {
    if cfg!(windows) {
        unix.replace("/", "\\")
    } else {
        unix.to_string()
    }
}

fn sort_lines(lines: &str) -> String {
    let mut lines: Vec<String> =
        lines.trim().lines().map(|s| s.to_owned()).collect();
    lines.sort();
    format!("{}\n", lines.join("\n"))
}

sherlock!(single_file, |wd: WorkDir, mut cmd| {
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(dir, "Sherlock", ".", |wd: WorkDir, mut cmd| {
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(line_numbers, |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-n");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
1:For the Doctor Watsons of this world, as opposed to the Sherlock
3:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(columns, |wd: WorkDir, mut cmd: Command| {
    cmd.arg("--column");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
57:For the Doctor Watsons of this world, as opposed to the Sherlock
49:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(with_filename, |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-H");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(with_heading, |wd: WorkDir, mut cmd: Command| {
    // This forces the issue since --with-filename is disabled by default
    // when searching one fil.e
    cmd.arg("--with-filename").arg("--heading");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
sherlock
For the Doctor Watsons of this world, as opposed to the Sherlock
be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(with_heading_default, "Sherlock", ".",
|wd: WorkDir, mut cmd: Command| {
    // Search two or more and get --with-filename enabled by default.
    // Use -j1 to get deterministic results.
    wd.create("foo", "Sherlock Holmes lives on Baker Street.");
    cmd.arg("-j1").arg("--heading");
    let lines: String = wd.stdout(&mut cmd);
    let expected1 = "\
foo
Sherlock Holmes lives on Baker Street.

sherlock
For the Doctor Watsons of this world, as opposed to the Sherlock
be, to a very large extent, the result of luck. Sherlock Holmes
";
    let expected2 = "\
sherlock
For the Doctor Watsons of this world, as opposed to the Sherlock
be, to a very large extent, the result of luck. Sherlock Holmes

foo
Sherlock Holmes lives on Baker Street.
";
    if lines != expected1 {
        assert_eq!(lines, expected2);
    } else {
        assert_eq!(lines, expected1);
    }
});

sherlock!(inverted, |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-v");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
Holmeses, success in the province of detective work must always
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
";
    assert_eq!(lines, expected);
});

sherlock!(inverted_line_numbers, |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-n").arg("-v");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
2:Holmeses, success in the province of detective work must always
4:can extract a clew from a wisp of straw or a flake of cigar ash;
5:but Doctor Watson has to have it taken out for him and dusted,
6:and exhibited clearly, with a label attached.
";
    assert_eq!(lines, expected);
});

sherlock!(case_insensitive, "sherlock", |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-i");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(word, "as", |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-w");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
";
    assert_eq!(lines, expected);
});

sherlock!(literal, "()", "file", |wd: WorkDir, mut cmd: Command| {
    wd.create("file", "blib\n()\nblab\n");
    cmd.arg("-F");
    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "()\n");
});

sherlock!(quiet, |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-q");
    let lines: String = wd.stdout(&mut cmd);
    assert!(lines.is_empty());
});

sherlock!(replace, |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-r").arg("FooBar");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
For the Doctor Watsons of this world, as opposed to the FooBar
be, to a very large extent, the result of luck. FooBar Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(replace_groups, "([A-Z][a-z]+) ([A-Z][a-z]+)",
|wd: WorkDir, mut cmd: Command| {
    cmd.arg("-r").arg("$2, $1");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
For the Watsons, Doctor of this world, as opposed to the Sherlock
be, to a very large extent, the result of luck. Holmes, Sherlock
but Watson, Doctor has to have it taken out for him and dusted,
";
    assert_eq!(lines, expected);
});

sherlock!(replace_named_groups, "(?P<first>[A-Z][a-z]+) (?P<last>[A-Z][a-z]+)",
|wd: WorkDir, mut cmd: Command| {
    cmd.arg("-r").arg("$last, $first");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
For the Watsons, Doctor of this world, as opposed to the Sherlock
be, to a very large extent, the result of luck. Holmes, Sherlock
but Watson, Doctor has to have it taken out for him and dusted,
";
    assert_eq!(lines, expected);
});

sherlock!(file_types, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create("file.py", "Sherlock");
    wd.create("file.rs", "Sherlock");
    cmd.arg("-t").arg("rust");
    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "file.rs:Sherlock\n");
});

sherlock!(file_types_all, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create("file.py", "Sherlock");
    cmd.arg("-t").arg("all");
    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "file.py:Sherlock\n");
});

sherlock!(file_types_negate, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.remove("sherlock");
    wd.create("file.py", "Sherlock");
    wd.create("file.rs", "Sherlock");
    cmd.arg("-T").arg("rust");
    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "file.py:Sherlock\n");
});

sherlock!(file_types_negate_all, "Sherlock", ".",
|wd: WorkDir, mut cmd: Command| {
    wd.create("file.py", "Sherlock");
    cmd.arg("-T").arg("all");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "\
sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
");
});

sherlock!(file_type_clear, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create("file.py", "Sherlock");
    wd.create("file.rs", "Sherlock");
    cmd.arg("--type-clear").arg("rust").arg("-t").arg("rust");
    wd.assert_err(&mut cmd);
});

sherlock!(file_type_add, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create("file.py", "Sherlock");
    wd.create("file.rs", "Sherlock");
    wd.create("file.wat", "Sherlock");
    cmd.arg("--type-add").arg("wat:*.wat").arg("-t").arg("wat");
    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "file.wat:Sherlock\n");
});

sherlock!(glob, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create("file.py", "Sherlock");
    wd.create("file.rs", "Sherlock");
    cmd.arg("-g").arg("*.rs");
    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "file.rs:Sherlock\n");
});

sherlock!(glob_negate, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.remove("sherlock");
    wd.create("file.py", "Sherlock");
    wd.create("file.rs", "Sherlock");
    cmd.arg("-g").arg("!*.rs");
    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "file.py:Sherlock\n");
});

sherlock!(count, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    cmd.arg("--count");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "sherlock:2\n";
    assert_eq!(lines, expected);
});

sherlock!(files_with_matches, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    cmd.arg("--files-with-matches");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "sherlock\n";
    assert_eq!(lines, expected);
});

sherlock!(after_context, |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-A").arg("1");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
";
    assert_eq!(lines, expected);
});

sherlock!(after_context_line_numbers, |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-A").arg("1").arg("-n");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
1:For the Doctor Watsons of this world, as opposed to the Sherlock
2-Holmeses, success in the province of detective work must always
3:be, to a very large extent, the result of luck. Sherlock Holmes
4-can extract a clew from a wisp of straw or a flake of cigar ash;
";
    assert_eq!(lines, expected);
});

sherlock!(before_context, |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-B").arg("1");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(before_context_line_numbers, |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-B").arg("1").arg("-n");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
1:For the Doctor Watsons of this world, as opposed to the Sherlock
2-Holmeses, success in the province of detective work must always
3:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(context, "world|attached", |wd: WorkDir, mut cmd: Command| {
    cmd.arg("-C").arg("1");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
--
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
";
    assert_eq!(lines, expected);
});

sherlock!(context_line_numbers, "world|attached",
|wd: WorkDir, mut cmd: Command| {
    cmd.arg("-C").arg("1").arg("-n");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
1:For the Doctor Watsons of this world, as opposed to the Sherlock
2-Holmeses, success in the province of detective work must always
--
5-but Doctor Watson has to have it taken out for him and dusted,
6:and exhibited clearly, with a label attached.
";
    assert_eq!(lines, expected);
});

sherlock!(ignore_hidden, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.remove("sherlock");
    wd.create(".sherlock", hay::SHERLOCK);
    wd.assert_err(&mut cmd);
});

sherlock!(no_ignore_hidden, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.remove("sherlock");
    wd.create(".sherlock", hay::SHERLOCK);

    cmd.arg("--hidden");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
.sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
.sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(ignore_git, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".gitignore", "sherlock\n");
    wd.assert_err(&mut cmd);
});

sherlock!(ignore_generic, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".ignore", "sherlock\n");
    wd.assert_err(&mut cmd);
});

sherlock!(ignore_ripgrep, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".rgignore", "sherlock\n");
    wd.assert_err(&mut cmd);
});

sherlock!(no_ignore, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".gitignore", "sherlock\n");
    cmd.arg("--no-ignore");
    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(ignore_git_parent, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.remove("sherlock");
    wd.create(".gitignore", "sherlock\n");
    wd.create_dir(".git");
    wd.create_dir("foo");
    wd.create("foo/sherlock", hay::SHERLOCK);
    // Even though we search in foo/, which has no .gitignore, ripgrep will
    // search parent directories and respect the gitignore files found.
    cmd.current_dir(wd.path().join("foo"));
    wd.assert_err(&mut cmd);
});

sherlock!(ignore_git_parent_stop, "Sherlock", ".",
|wd: WorkDir, mut cmd: Command| {
    // This tests that searching parent directories for .gitignore files stops
    // after it sees a .git directory. To test this, we create this directory
    // hierarchy:
    //
    // .gitignore (contains `sherlock`)
    // foo/
    //   .git
    //   bar/
    //      sherlock
    //
    // And we perform the search inside `foo/bar/`. ripgrep will stop looking
    // for .gitignore files after it sees `foo/.git/`, and therefore not
    // respect the top-level `.gitignore` containing `sherlock`.
    wd.remove("sherlock");
    wd.create(".gitignore", "sherlock\n");
    wd.create_dir("foo");
    wd.create_dir("foo/.git");
    wd.create_dir("foo/bar");
    wd.create("foo/bar/sherlock", hay::SHERLOCK);
    cmd.current_dir(wd.path().join("foo").join("bar"));

    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(ignore_ripgrep_parent_no_stop, "Sherlock", ".",
|wd: WorkDir, mut cmd: Command| {
    // This is like the `ignore_git_parent_stop` test, except it checks that
    // ripgrep *doesn't* stop checking for .rgignore files.
    wd.remove("sherlock");
    wd.create(".rgignore", "sherlock\n");
    wd.create_dir("foo");
    wd.create_dir("foo/.git");
    wd.create_dir("foo/bar");
    wd.create("foo/bar/sherlock", hay::SHERLOCK);
    cmd.current_dir(wd.path().join("foo").join("bar"));
    // The top-level .rgignore applies.
    wd.assert_err(&mut cmd);
});

sherlock!(no_parent_ignore_git, "Sherlock", ".",
|wd: WorkDir, mut cmd: Command| {
    // Set up a directory hierarchy like this:
    //
    // .gitignore
    // foo/
    //   .gitignore
    //   sherlock
    //   watson
    //
    // Where `.gitignore` contains `sherlock` and `foo/.gitignore` contains
    // `watson`.
    //
    // Now *do the search* from the foo directory. By default, ripgrep will
    // search parent directories for .gitignore files. The --no-ignore-parent
    // flag should prevent that. At the same time, the `foo/.gitignore` file
    // will still be respected (since the search is happening in `foo/`).
    //
    // In other words, we should only see results from `sherlock`, not from
    // `watson`.
    wd.remove("sherlock");
    wd.create(".gitignore", "sherlock\n");
    wd.create_dir("foo");
    wd.create("foo/.gitignore", "watson\n");
    wd.create("foo/sherlock", hay::SHERLOCK);
    wd.create("foo/watson", hay::SHERLOCK);
    cmd.current_dir(wd.path().join("foo"));
    cmd.arg("--no-ignore-parent");

    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(symlink_nofollow, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.remove("sherlock");
    wd.create_dir("foo");
    wd.create_dir("foo/bar");
    wd.link("foo/baz", "foo/bar/baz");
    wd.create_dir("foo/baz");
    wd.create("foo/baz/sherlock", hay::SHERLOCK);
    cmd.current_dir(wd.path().join("foo/bar"));
    wd.assert_err(&mut cmd);
});

sherlock!(symlink_follow, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.remove("sherlock");
    wd.create_dir("foo");
    wd.create_dir("foo/bar");
    wd.create_dir("foo/baz");
    wd.create("foo/baz/sherlock", hay::SHERLOCK);
    wd.link("foo/baz", "foo/bar/baz");
    cmd.arg("-L");
    cmd.current_dir(wd.path().join("foo/bar"));

    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
baz/sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
baz/sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, path(expected));
});

sherlock!(unrestricted1, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".gitignore", "sherlock\n");
    cmd.arg("-u");

    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

sherlock!(unrestricted2, "Sherlock", ".", |wd: WorkDir, mut cmd: Command| {
    wd.remove("sherlock");
    wd.create(".sherlock", hay::SHERLOCK);
    cmd.arg("-uu");

    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
.sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
.sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

#[cfg(not(windows))]
sherlock!(unrestricted3, "foo", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create("file", "foo\x00bar\nfoo\x00baz\n");
    cmd.arg("-uuu");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "file:foo\x00bar\nfile:foo\x00baz\n");
});

// On Windows, this test uses memory maps, so the NUL bytes don't get replaced.
#[cfg(windows)]
sherlock!(unrestricted3, "foo", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create("file", "foo\x00bar\nfoo\x00baz\n");
    cmd.arg("-uuu");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "file:foo\x00bar\nfile:foo\x00baz\n");
});

sherlock!(vimgrep, "Sherlock|Watson", ".", |wd: WorkDir, mut cmd: Command| {
    cmd.arg("--vimgrep");

    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
sherlock:1:16:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:1:57:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:3:49:be, to a very large extent, the result of luck. Sherlock Holmes
sherlock:5:12:but Doctor Watson has to have it taken out for him and dusted,
";
    assert_eq!(lines, expected);
});

// See: https://github.com/BurntSushi/ripgrep/issues/16
clean!(regression_16, "xyz", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".gitignore", "ghi/");
    wd.create_dir("ghi");
    wd.create_dir("def/ghi");
    wd.create("ghi/toplevel.txt", "xyz");
    wd.create("def/ghi/subdir.txt", "xyz");
    wd.assert_err(&mut cmd);
});

// See: https://github.com/BurntSushi/ripgrep/issues/25
clean!(regression_25, "test", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".gitignore", "/llvm/");
    wd.create_dir("src/llvm");
    wd.create("src/llvm/foo", "test");

    let lines: String = wd.stdout(&mut cmd);
    let expected = path("src/llvm/foo:test\n");
    assert_eq!(lines, expected);

    cmd.current_dir(wd.path().join("src"));
    let lines: String = wd.stdout(&mut cmd);
    let expected = path("llvm/foo:test\n");
    assert_eq!(lines, expected);
});

// See: https://github.com/BurntSushi/ripgrep/issues/30
clean!(regression_30, "test", ".", |wd: WorkDir, mut cmd: Command| {
    if cfg!(windows) {
        wd.create(".gitignore", "vendor/**\n!vendor\\manifest");
    } else {
        wd.create(".gitignore", "vendor/**\n!vendor/manifest");
    }
    wd.create_dir("vendor");
    wd.create("vendor/manifest", "test");

    let lines: String = wd.stdout(&mut cmd);
    let expected = path("vendor/manifest:test\n");
    assert_eq!(lines, expected);
});

// See: https://github.com/BurntSushi/ripgrep/issues/49
clean!(regression_49, "xyz", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".gitignore", "foo/bar");
    wd.create_dir("test/foo/bar");
    wd.create("test/foo/bar/baz", "test");
    wd.assert_err(&mut cmd);
});

// See: https://github.com/BurntSushi/ripgrep/issues/50
clean!(regression_50, "xyz", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".gitignore", "XXX/YYY/");
    wd.create_dir("abc/def/XXX/YYY");
    wd.create_dir("ghi/XXX/YYY");
    wd.create("abc/def/XXX/YYY/bar", "test");
    wd.create("ghi/XXX/YYY/bar", "test");
    wd.assert_err(&mut cmd);
});

// See: https://github.com/BurntSushi/ripgrep/issues/65
clean!(regression_65, "xyz", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".gitignore", "a/");
    wd.create_dir("a");
    wd.create("a/foo", "xyz");
    wd.create("a/bar", "xyz");
    wd.assert_err(&mut cmd);
});

// See: https://github.com/BurntSushi/ripgrep/issues/67
clean!(regression_67, "test", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".gitignore", "/*\n!/dir");
    wd.create_dir("dir");
    wd.create_dir("foo");
    wd.create("foo/bar", "test");
    wd.create("dir/bar", "test");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, path("dir/bar:test\n"));
});

// See: https://github.com/BurntSushi/ripgrep/issues/90
clean!(regression_90, "test", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".gitignore", "!.foo");
    wd.create(".foo", "test");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, ".foo:test\n");
});

// See: https://github.com/BurntSushi/ripgrep/issues/93
clean!(regression_93, r"(\d{1,3}\.){3}\d{1,3}", ".",
|wd: WorkDir, mut cmd: Command| {
    wd.create("foo", "192.168.1.1");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "foo:192.168.1.1\n");
});

// See: https://github.com/BurntSushi/ripgrep/issues/99
clean!(regression_99, "test", ".",
|wd: WorkDir, mut cmd: Command| {
    wd.create("foo1", "test");
    wd.create("foo2", "zzz");
    wd.create("bar", "test");
    cmd.arg("-j1").arg("--heading");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(sort_lines(&lines), sort_lines("bar\ntest\n\nfoo1\ntest\n"));
});

// See: https://github.com/BurntSushi/ripgrep/issues/105
clean!(regression_105_part1, "test", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create("foo", "zztest");
    cmd.arg("--vimgrep");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "foo:1:3:zztest\n");
});

// See: https://github.com/BurntSushi/ripgrep/issues/105
clean!(regression_105_part2, "test", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create("foo", "zztest");
    cmd.arg("--column");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "foo:3:zztest\n");
});

// See: https://github.com/BurntSushi/ripgrep/issues/20
sherlock!(feature_20_no_filename, "Sherlock", ".",
|wd: WorkDir, mut cmd: Command| {
    cmd.arg("--no-filename");

    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

// See: https://github.com/BurntSushi/ripgrep/issues/68
clean!(feature_68_no_ignore_vcs, "test", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".gitignore", "foo");
    wd.create(".ignore", "bar");
    wd.create("foo", "test");
    wd.create("bar", "test");
    cmd.arg("--no-ignore-vcs");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "foo:test\n");
});

// See: https://github.com/BurntSushi/ripgrep/issues/70
sherlock!(feature_70_smart_case, "sherlock", ".",
|wd: WorkDir, mut cmd: Command| {
    cmd.arg("--smart-case");

    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(lines, expected);
});

// See: https://github.com/BurntSushi/ripgrep/issues/89
sherlock!(feature_89_files_with_matches, "Sherlock", ".",
|wd: WorkDir, mut cmd: Command| {
    cmd.arg("--null").arg("--files-with-matches");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "sherlock\x00");
});

// See: https://github.com/BurntSushi/ripgrep/issues/89
sherlock!(feature_89_count, "Sherlock", ".",
|wd: WorkDir, mut cmd: Command| {
    cmd.arg("--null").arg("--count");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "sherlock\x002\n");
});

// See: https://github.com/BurntSushi/ripgrep/issues/89
sherlock!(feature_89_files, "NADA", ".",
|wd: WorkDir, mut cmd: Command| {
    cmd.arg("--null").arg("--files");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "sherlock\x00");
});

// See: https://github.com/BurntSushi/ripgrep/issues/89
sherlock!(feature_89_match, "Sherlock", ".",
|wd: WorkDir, mut cmd: Command| {
    cmd.arg("--null").arg("-C1");

    let lines: String = wd.stdout(&mut cmd);
    let expected = "\
sherlock\x00For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock\x00Holmeses, success in the province of detective work must always
sherlock\x00be, to a very large extent, the result of luck. Sherlock Holmes
sherlock\x00can extract a clew from a wisp of straw or a flake of cigar ash;
";
    assert_eq!(lines, expected);
});

// See: https://github.com/BurntSushi/ripgrep/issues/109
clean!(feature_109_max_depth, "far", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create_dir("one");
    wd.create("one/pass", "far");
    wd.create_dir("one/too");
    wd.create("one/too/many", "far");

    cmd.arg("--maxdepth").arg("2");

    let lines: String = wd.stdout(&mut cmd);
    let expected = path("one/pass:far\n");
    assert_eq!(lines, expected);
});

// See: https://github.com/BurntSushi/ripgrep/issues/124
clean!(feature_109_case_sensitive_part1, "test", ".",
|wd: WorkDir, mut cmd: Command| {
    wd.create("foo", "tEsT");
    cmd.arg("--smart-case").arg("--case-sensitive");
    wd.assert_err(&mut cmd);
});

// See: https://github.com/BurntSushi/ripgrep/issues/124
clean!(feature_109_case_sensitive_part2, "test", ".",
|wd: WorkDir, mut cmd: Command| {
    wd.create("foo", "tEsT");
    cmd.arg("--ignore-case").arg("--case-sensitive");
    wd.assert_err(&mut cmd);
});

#[test]
fn binary_nosearch() {
    let wd = WorkDir::new("binary_nosearch");
    wd.create("file", "foo\x00bar\nfoo\x00baz\n");
    let mut cmd = wd.command();
    cmd.arg("foo").arg("file");
    wd.assert_err(&mut cmd);
}

// The following two tests show a discrepancy in search results between
// searching with memory mapped files and stream searching. Stream searching
// uses a heuristic (that GNU grep also uses) where NUL bytes are replaced with
// the EOL terminator, which tends to avoid allocating large amounts of memory
// for really long "lines." The memory map searcher has no need to worry about
// such things, and more than that, it would be pretty hard for it to match
// the semantics of streaming search in this case.
//
// Binary files with lots of NULs aren't really part of the use case of ripgrep
// (or any other grep-like tool for that matter), so we shouldn't feel too bad
// about it.
#[test]
fn binary_search_mmap() {
    let wd = WorkDir::new("binary_search_mmap");
    wd.create("file", "foo\x00bar\nfoo\x00baz\n");
    let mut cmd = wd.command();
    cmd.arg("-a").arg("--mmap").arg("foo").arg("file");
    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "foo\x00bar\nfoo\x00baz\n");
}

#[test]
fn binary_search_no_mmap() {
    let wd = WorkDir::new("binary_search_no_mmap");
    wd.create("file", "foo\x00bar\nfoo\x00baz\n");
    let mut cmd = wd.command();
    cmd.arg("-a").arg("--no-mmap").arg("foo").arg("file");
    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "foo\x00bar\nfoo\x00baz\n");
}

#[test]
fn files() {
    let wd = WorkDir::new("files");
    wd.create("file", "");
    wd.create_dir("dir");
    wd.create("dir/file", "");

    let mut cmd = wd.command();
    cmd.arg("--files");
    let lines: String = wd.stdout(&mut cmd);
    assert!(lines == path("file\ndir/file\n")
            || lines == path("dir/file\nfile\n"));
}

// See: https://github.com/BurntSushi/ripgrep/issues/64
#[test]
fn regression_64() {
    let wd = WorkDir::new("regression_64");
    wd.create_dir("dir");
    wd.create_dir("foo");
    wd.create("dir/abc", "");
    wd.create("foo/abc", "");

    let mut cmd = wd.command();
    cmd.arg("--files").arg("foo");
    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, path("foo/abc\n"));
}

#[test]
fn type_list() {
    let wd = WorkDir::new("type_list");

    let mut cmd = wd.command();
    cmd.arg("--type-list");
    let lines: String = wd.stdout(&mut cmd);
    // This can change over time, so just make sure we print something.
    assert!(!lines.is_empty());
}
