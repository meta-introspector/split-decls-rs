macro_rules! deps {
    () => {
        Error!();
        Walk!();
        Result!();
    };
}

macro_rules! copy_template {
    () => {
        deps!();
        # [doc = " Copy a template into a [`DirRoot`][super::DirRoot]"] # [doc = ""] # [doc = " Note: Generally you'll use [`DirRoot::with_template`][super::DirRoot::with_template] instead."] # [doc = ""] # [doc = " Note: Ignores `.keep` files"] # [cfg (feature = "dir")] pub fn copy_template (source : impl AsRef < std :: path :: Path > , dest : impl AsRef < std :: path :: Path > ,) -> Result < () , crate :: assert :: Error > { let source = source . as_ref () ; let dest = dest . as_ref () ; let source = canonicalize (source) . map_err (| e | format ! ("Failed to canonicalize {}: {}" , source . display () , e)) ? ; std :: fs :: create_dir_all (dest) . map_err (| e | format ! ("Failed to create {}: {}" , dest . display () , e)) ? ; let dest = canonicalize (dest) . map_err (| e | format ! ("Failed to canonicalize {}: {}" , dest . display () , e)) ? ; for current in Walk :: new (& source) { let current = current . map_err (| e | e . to_string ()) ? ; let rel = current . strip_prefix (& source) . unwrap () ; let target = dest . join (rel) ; shallow_copy (& current , & target) ? ; } Ok (()) }
    };
}

copy_template!();