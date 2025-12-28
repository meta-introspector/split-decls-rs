macro_rules! temp_dir {
    () => {
        # [doc = " Returns the default temporary directory, used for both temporary directories and files if no"] # [doc = " directory is explicitly specified."] # [doc = ""] # [doc = " This function simply delegates to [`std::env::temp_dir`] unless the default temporary directory"] # [doc = " has been override by a call to [`override_temp_dir`]."] # [doc = ""] # [doc = " **NOTE:** This function does not check if the returned directory exists and/or is writable."] pub fn temp_dir () -> PathBuf { DEFAULT_TEMPDIR . get () . map (| p | p . to_owned ()) . unwrap_or_else (env :: temp_dir) }
    };
}

temp_dir!();