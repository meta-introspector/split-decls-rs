macro_rules! deps {
    () => {
        FileSet!();
        AnchoredPath!();
        FileId!();
        VfsPath!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl FileSet { # [doc = " Returns the number of stored paths."] pub fn len (& self) -> usize { self . files . len () } # [doc = " Get the id of the file corresponding to `path`."] # [doc = ""] # [doc = " If either `path`'s [`anchor`](AnchoredPath::anchor) or the resolved path is not in"] # [doc = " the set, returns [`None`]."] pub fn resolve_path (& self , path : AnchoredPath < '_ >) -> Option < FileId > { let mut base = self . paths [& path . anchor] . clone () ; base . pop () ; let path = base . join (path . path) ? ; self . files . get (& path) . copied () } # [doc = " Get the id corresponding to `path` if it exists in the set."] pub fn file_for_path (& self , path : & VfsPath) -> Option < & FileId > { self . files . get (path) } # [doc = " Get the path corresponding to `file` if it exists in the set."] pub fn path_for_file (& self , file : & FileId) -> Option < & VfsPath > { self . paths . get (file) } # [doc = " Insert the `file_id, path` pair into the set."] # [doc = ""] # [doc = " # Note"] # [doc = " Multiple [`FileId`] can be mapped to the same [`VfsPath`], and vice-versa."] pub fn insert (& mut self , file_id : FileId , path : VfsPath) { self . files . insert (path . clone () , file_id) ; self . paths . insert (file_id , path) ; } # [doc = " Iterate over this set's ids."] pub fn iter (& self) -> impl Iterator < Item = FileId > + '_ { self . paths . keys () . copied () } }
    };
}

impl_4!();