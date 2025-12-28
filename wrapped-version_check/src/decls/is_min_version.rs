macro_rules! is_min_version {
    () => {
        # [doc = " Checks that the running or installed `rustc` is **at least** some minimum"] # [doc = " version."] # [doc = ""] # [doc = " The format of `min_version` is a semantic version: `1.3.0`, `1.15.0-beta`,"] # [doc = " `1.14.0`, `1.16.0-nightly`, etc."] # [doc = ""] # [doc = " If the version cannot be retrieved or parsed, or if `min_version` could not"] # [doc = " be parsed, returns `None`. Otherwise returns `true` if the installed `rustc`"] # [doc = " is at least `min_version` and `false` otherwise."] pub fn is_min_version (min_version : & str) -> Option < bool > { match (Version :: read () , Version :: parse (min_version)) { (Some (rustc_ver) , Some (min_ver)) => Some (rustc_ver >= min_ver) , _ => None , } }
    };
}

is_min_version!()