macro_rules! deps {
    () => {
        FileSet!();
        Vfs!();
        FileSetConfigBuilder!();
        VfsPath!();
        PrefixOf!();
        FileSetConfig!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl FileSetConfig { # [doc = " Returns a builder for `FileSetConfig`."] pub fn builder () -> FileSetConfigBuilder { FileSetConfigBuilder :: default () } # [doc = " Partition `vfs` into `FileSet`s."] # [doc = ""] # [doc = " Creates a new [`FileSet`] for every set of prefixes in `self`."] pub fn partition (& self , vfs : & Vfs) -> Vec < FileSet > { let mut scratch_space = Vec :: new () ; let mut res = vec ! [FileSet :: default () ; self . len ()] ; for (file_id , path) in vfs . iter () { let root = self . classify (path , & mut scratch_space) ; res [root] . insert (file_id , path . clone ()) ; } res } # [doc = " Number of sets that `self` can partition a [`Vfs`] into."] fn len (& self) -> usize { self . n_file_sets } # [doc = " Get the lexicographically ordered vector of the underlying map."] pub fn roots (& self) -> Vec < (Vec < u8 > , u64) > { self . map . stream () . into_byte_vec () } # [doc = " Returns the set index for the given `path`."] # [doc = ""] # [doc = " `scratch_space` is used as a buffer and will be entirely replaced."] fn classify (& self , path : & VfsPath , scratch_space : & mut Vec < u8 >) -> usize { let path = path . parent () . unwrap_or_else (| | path . clone ()) ; scratch_space . clear () ; path . encode (scratch_space) ; let automaton = PrefixOf :: new (scratch_space . as_slice ()) ; let mut longest_prefix = self . len () - 1 ; let mut stream = self . map . search (automaton) . into_stream () ; while let Some ((_ , v)) = stream . next () { longest_prefix = v as usize ; } longest_prefix } }
    };
}

impl_8!()