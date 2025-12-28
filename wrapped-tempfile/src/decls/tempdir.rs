macro_rules! deps {
    () => {
        TempDir!();
    };
}

macro_rules! tempdir {
    () => {
        deps!();
        # [doc = " Create a new temporary directory. Also see [`tempdir_in`]."] # [doc = ""] # [doc = " The `tempdir` function creates a directory in the file system and returns a"] # [doc = " [`TempDir`]. The directory will be automatically deleted when the `TempDir`'s"] # [doc = " destructor is run."] # [doc = ""] # [doc = " # Resource Leaking"] # [doc = ""] # [doc = " See [the resource leaking][resource-leaking] docs on `TempDir`."] # [doc = ""] # [doc = " # Security"] # [doc = ""] # [doc = " Temporary directories are created with the default permissions unless otherwise"] # [doc = " specified via [`Builder::permissions`]. Depending on your platform, this may make"] # [doc = " them world-readable."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If the directory can not be created, `Err` is returned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tempfile::tempdir;"] # [doc = " use std::fs::File;"] # [doc = " use std::io::Write;"] # [doc = ""] # [doc = " // Create a directory inside of `env::temp_dir()`"] # [doc = " let tmp_dir = tempdir()?;"] # [doc = ""] # [doc = " let file_path = tmp_dir.path().join(\"my-temporary-note.txt\");"] # [doc = " let mut tmp_file = File::create(file_path)?;"] # [doc = " writeln!(tmp_file, \"Brian was here. Briefly.\")?;"] # [doc = ""] # [doc = " // `tmp_dir` goes out of scope, the directory as well as"] # [doc = " // `tmp_file` will be deleted here."] # [doc = " drop(tmp_file);"] # [doc = " tmp_dir.close()?;"] # [doc = " # Ok::<(), std::io::Error>(())"] # [doc = " ```"] # [doc = ""] # [doc = " [`TempDir`]: struct.TempDir.html"] # [doc = " [resource-leaking]: struct.TempDir.html#resource-leaking"] pub fn tempdir () -> io :: Result < TempDir > { TempDir :: new () }
    };
}

tempdir!()