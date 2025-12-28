macro_rules! deps {
    () => {
        Vfs!();
        VfsPath!();
    };
}

macro_rules! FileSetConfig {
    () => {
        deps!();
        # [doc = " This contains path prefixes to partition a [`Vfs`] into [`FileSet`]s."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use vfs::{file_set::FileSetConfigBuilder, VfsPath, Vfs};"] # [doc = " let mut builder = FileSetConfigBuilder::default();"] # [doc = " builder.add_file_set(vec![VfsPath::new_virtual_path(\"/src\".to_string())]);"] # [doc = " let config = builder.build();"] # [doc = " let mut file_system = Vfs::default();"] # [doc = " file_system.set_file_contents(VfsPath::new_virtual_path(\"/src/main.rs\".to_string()), Some(vec![]));"] # [doc = " file_system.set_file_contents(VfsPath::new_virtual_path(\"/src/lib.rs\".to_string()), Some(vec![]));"] # [doc = " file_system.set_file_contents(VfsPath::new_virtual_path(\"/build.rs\".to_string()), Some(vec![]));"] # [doc = " // contains the sets :"] # [doc = " // { \"/src/main.rs\", \"/src/lib.rs\" }"] # [doc = " // { \"build.rs\" }"] # [doc = " let sets = config.partition(&file_system);"] # [doc = " ```"] # [derive (Debug)] pub struct FileSetConfig { # [doc = " Number of sets that `self` can partition a [`Vfs`] into."] # [doc = ""] # [doc = " This should be the number of sets in `self.map` + 1 for files that don't fit in any"] # [doc = " defined set."] n_file_sets : usize , # [doc = " Map from encoded paths to the set they belong to."] map : fst :: Map < Vec < u8 > > , }
    };
}

FileSetConfig!();