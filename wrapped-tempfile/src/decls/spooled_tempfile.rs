macro_rules! deps {
    () => {
        SpooledTempFile!();
    };
}

macro_rules! spooled_tempfile {
    () => {
        deps!();
        # [doc = " Create a new [`SpooledTempFile`]. Also see [`spooled_tempfile_in`]."] # [doc = ""] # [doc = " # Security"] # [doc = ""] # [doc = " This variant is secure/reliable in the presence of a pathological temporary"] # [doc = " file cleaner."] # [doc = ""] # [doc = " # Backing Storage"] # [doc = ""] # [doc = " By default, the underlying temporary file will be created in your operating system's temporary"] # [doc = " file directory which is _often_ an in-memory filesystem. You may want to consider using"] # [doc = " [`spooled_tempfile_in`] instead, passing a storage-backed filesystem (e.g., `/var/tmp` on"] # [doc = " Linux)."] # [doc = ""] # [doc = " # Resource Leaking"] # [doc = ""] # [doc = " The temporary file will be automatically removed by the OS when the last"] # [doc = " handle to it is closed. This doesn't rely on Rust destructors being run, so"] # [doc = " will (almost) never fail to clean up the temporary file."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tempfile::spooled_tempfile;"] # [doc = " use std::io::Write;"] # [doc = ""] # [doc = " let mut file = spooled_tempfile(15);"] # [doc = ""] # [doc = " writeln!(file, \"short line\")?;"] # [doc = " assert!(!file.is_rolled());"] # [doc = ""] # [doc = " // as a result of this write call, the size of the data will exceed"] # [doc = " // `max_size` (15), so it will be written to a temporary file on disk,"] # [doc = " // and the in-memory buffer will be dropped"] # [doc = " writeln!(file, \"marvin gardens\")?;"] # [doc = " assert!(file.is_rolled());"] # [doc = " # Ok::<(), std::io::Error>(())"] # [doc = " ```"] # [inline] pub fn spooled_tempfile (max_size : usize) -> SpooledTempFile { SpooledTempFile :: new (max_size) }
    };
}

spooled_tempfile!()