macro_rules! deps {
    () => {
        Creation!();
        Result!();
    };
}

macro_rules! scripted_fixture_read_only {
    () => {
        deps!();
        # [doc = " Run the executable at `script_name`, like `make_repo.sh` or `my_setup.py` to produce a read-only directory to which"] # [doc = " the path is returned."] # [doc = ""] # [doc = " Note that it persists and the script at `script_name` will only be executed once if it ran without error."] # [doc = ""] # [doc = " ### Automatic Archive Creation"] # [doc = ""] # [doc = " In order to speed up CI and even local runs should the cache get purged, the result of each script run"] # [doc = " is automatically placed into a compressed _tar_ archive."] # [doc = " If a script result doesn't exist, these will be checked first and extracted if present, which they are by default."] # [doc = " This behaviour can be prohibited by setting the `GIX_TEST_IGNORE_ARCHIVES` to any value."] # [doc = ""] # [doc = " To speed CI up, one can add these archives to the repository. Since LFS is not currently being used, it is"] # [doc = " important to check their size first, though in most cases generated archives will not be very large."] # [doc = ""] # [doc = " #### Disable Archive Creation"] # [doc = ""] # [doc = " If archives aren't useful, they can be disabled by using `.gitignore` specifications."] # [doc = " That way it's trivial to prevent creation of all archives with `generated-archives/*.tar{.xz}` in the root"] # [doc = " or more specific `.gitignore` configurations in lower levels of the work tree."] # [doc = ""] # [doc = " The latter is useful if the script's output is platform specific."] pub fn scripted_fixture_read_only (script_name : impl AsRef < Path >) -> Result < PathBuf > { scripted_fixture_read_only_with_args (script_name , None :: < String >) }
    };
}

scripted_fixture_read_only!();