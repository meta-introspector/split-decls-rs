// Generated macro for scripted_fixture_writable (function)
macro_rules! Depcratescripted_fixture_writable {
() => {
// Module: crate
// Provides: {"scripted_fixture_writable"}
// Dependencies: {}
# [doc = " Run the executable at `script_name`, like `make_repo.sh` to produce a writable directory to which"] # [doc = " the tempdir is returned. It will be removed automatically, courtesy of [`tempfile::TempDir`]."] # [doc = ""] # [doc = " Note that `script_name` is only executed once, so the data can be copied from its read-only location."] pub fn scripted_fixture_writable (script_name : impl AsRef < Path >) -> Result < tempfile :: TempDir > { scripted_fixture_writable_with_args (script_name , None :: < String > , Creation :: CopyFromReadOnly) }
};
}
