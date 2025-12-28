macro_rules! deps {
    () => {
        NamedTempFile!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl NamedTempFile < File > { # [doc = " Securely reopen the temporary file."] # [doc = ""] # [doc = " This function is useful when you need multiple independent handles to"] # [doc = " the same file. It's perfectly fine to drop the original `NamedTempFile`"] # [doc = " while holding on to `File`s returned by this function; the `File`s will"] # [doc = " remain usable. However, they may not be nameable."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If the file cannot be reopened, `Err` is returned."] # [doc = ""] # [doc = " # Security"] # [doc = ""] # [doc = " Unlike `File::open(my_temp_file.path())`, `NamedTempFile::reopen()`"] # [doc = " guarantees that the re-opened file is the _same_ file, even in the"] # [doc = " presence of pathological temporary file cleaners."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use tempfile::NamedTempFile;"] # [doc = ""] # [doc = " let file = NamedTempFile::new()?;"] # [doc = ""] # [doc = " let another_handle = file.reopen()?;"] # [doc = " # Ok::<(), std::io::Error>(())"] # [doc = " ```"] pub fn reopen (& self) -> io :: Result < File > { imp :: reopen (self . as_file () , NamedTempFile :: path (self)) . with_err_path (| | NamedTempFile :: path (self)) } }
    };
}

impl_51!()