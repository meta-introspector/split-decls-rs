macro_rules! deps {
    () => {
        VfsPath!();
        FileSetConfigBuilder!();
        FileSetConfig!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl FileSetConfigBuilder { # [doc = " Returns the number of sets currently held."] pub fn len (& self) -> usize { self . roots . len () } # [doc = " Add a new set of paths prefixes."] pub fn add_file_set (& mut self , roots : Vec < VfsPath >) { self . roots . push (roots) ; } # [doc = " Build the `FileSetConfig`."] pub fn build (self) -> FileSetConfig { let n_file_sets = self . roots . len () + 1 ; let map = { let mut entries = Vec :: new () ; for (i , paths) in self . roots . into_iter () . enumerate () { for p in paths { let mut buf = Vec :: new () ; p . encode (& mut buf) ; entries . push ((buf , i as u64)) ; } } entries . sort () ; entries . dedup_by (| (a , _) , (b , _) | a == b) ; fst :: Map :: from_iter (entries) . unwrap () } ; FileSetConfig { n_file_sets , map } } }
    };
}

impl_10!();