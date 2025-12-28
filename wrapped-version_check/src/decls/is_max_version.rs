macro_rules! is_max_version {
    () => {
        # [doc = " Checks that the running or installed `rustc` is **at most** some maximum"] # [doc = " version."] # [doc = ""] # [doc = " The format of `max_version` is a semantic version: `1.3.0`, `1.15.0-beta`,"] # [doc = " `1.14.0`, `1.16.0-nightly`, etc."] # [doc = ""] # [doc = " If the version cannot be retrieved or parsed, or if `max_version` could not"] # [doc = " be parsed, returns `None`. Otherwise returns `true` if the installed `rustc`"] # [doc = " is at most `max_version` and `false` otherwise."] pub fn is_max_version (max_version : & str) -> Option < bool > { match (Version :: read () , Version :: parse (max_version)) { (Some (rustc_ver) , Some (max_ver)) => Some (rustc_ver <= max_ver) , _ => None , } }
    };
}

is_max_version!()