macro_rules! deps {
    () => {
        TempDir!();
    };
}

macro_rules! tempdir_in {
    () => {
        deps!();
        # [doc = " Create a new temporary directory in a specific directory. Also see [`tempdir`]."] # [doc = ""] # [doc = " The `tempdir_in` function creates a directory in the specified directory"] # [doc = " and returns a [`TempDir`]."] # [doc = " The directory will be automatically deleted when the `TempDir`s"] # [doc = " destructor is run."] # [doc = ""] # [doc = " # Resource Leaking"] # [doc = ""] # [doc = " See [the resource leaking][resource-leaking] docs on `TempDir`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If the directory can not be created, `Err` is returned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tempfile::tempdir_in;"] # [doc = " use std::fs::File;"] # [doc = " use std::io::Write;"] # [doc = ""] # [doc = " // Create a directory inside of the current directory."] # [doc = " let tmp_dir = tempdir_in(\".\")?;"] # [doc = ""] # [doc = " let file_path = tmp_dir.path().join(\"my-temporary-note.txt\");"] # [doc = " let mut tmp_file = File::create(file_path)?;"] # [doc = " writeln!(tmp_file, \"Brian was here. Briefly.\")?;"] # [doc = ""] # [doc = " // `tmp_dir` goes out of scope, the directory as well as"] # [doc = " // `tmp_file` will be deleted here."] # [doc = " drop(tmp_file);"] # [doc = " tmp_dir.close()?;"] # [doc = " # Ok::<(), std::io::Error>(())"] # [doc = " ```"] # [doc = ""] # [doc = " [`TempDir`]: struct.TempDir.html"] # [doc = " [resource-leaking]: struct.TempDir.html#resource-leaking"] pub fn tempdir_in < P : AsRef < Path > > (dir : P) -> io :: Result < TempDir > { TempDir :: new_in (dir) }
    };
}

tempdir_in!();