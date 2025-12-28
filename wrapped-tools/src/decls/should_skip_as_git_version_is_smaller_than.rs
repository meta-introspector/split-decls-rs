macro_rules! should_skip_as_git_version_is_smaller_than {
    () => {
        # [doc = " Returns true if the given `major`, `minor` and `patch` is smaller than the actual git version on the system"] # [doc = " to facilitate skipping a test on the caller."] # [doc = " Will never return true on CI which is expected to have a recent enough git version."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `git` cannot be executed or if its version output cannot be parsed."] pub fn should_skip_as_git_version_is_smaller_than (major : u8 , minor : u8 , patch : u8) -> bool { if is_ci :: cached () { return false ; } * GIT_VERSION < (major , minor , patch) }
    };
}

should_skip_as_git_version_is_smaller_than!();