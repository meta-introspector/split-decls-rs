macro_rules! Creation {
    () => {
        # [doc = " Define how [`scripted_fixture_writable_with_args()`] uses produces the writable copy."] pub enum Creation { # [doc = " Run the script once and copy the data from its output to the writable location."] # [doc = " This is fast but won't work if absolute paths are produced by the script."] # [doc = ""] # [doc = " ### Limitation"] # [doc = ""] # [doc = " Cannot handle symlinks currently. Waiting for [this PR](https://github.com/webdesus/fs_extra/pull/70)."] CopyFromReadOnly , # [doc = " Run the script in the writable location. That way, absolute paths match the location."] ExecuteScript , }
    };
}

Creation!();