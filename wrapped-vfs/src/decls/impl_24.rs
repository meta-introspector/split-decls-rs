macro_rules! deps {
    () => {
        Directories!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl Directories { # [doc = " Returns `true` if `path` is included in `self`."] pub fn contains_file (& self , path : & AbsPath) -> bool { let ext = path . extension () . unwrap_or_default () ; if self . extensions . iter () . all (| it | it . as_str () != ext) { return false ; } self . includes_path (path) } # [doc = " Returns `true` if `path` is included in `self`."] # [doc = ""] # [doc = " Since `path` is supposed to be a directory, this will not take extension"] # [doc = " into account."] pub fn contains_dir (& self , path : & AbsPath) -> bool { self . includes_path (path) } # [doc = " Returns `true` if `path` is included in `self`."] # [doc = ""] # [doc = " It is included if"] # [doc = "   - An element in `self.include` is a prefix of `path`."] # [doc = "   - This path is longer than any element in `self.exclude` that is a prefix"] # [doc = "     of `path`. In case of equality, exclusion wins."] fn includes_path (& self , path : & AbsPath) -> bool { let mut include : Option < & AbsPathBuf > = None ; for incl in & self . include { if path . starts_with (incl) { include = Some (match include { Some (prev) if prev . starts_with (incl) => prev , _ => incl , }) ; } } let include = match include { Some (it) => it , None => return false , } ; ! self . exclude . iter () . any (| excl | path . starts_with (excl) && excl . starts_with (include)) } }
    };
}

impl_24!();