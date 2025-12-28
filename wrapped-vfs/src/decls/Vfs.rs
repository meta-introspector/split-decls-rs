macro_rules! deps {
    () => {
        FileState!();
        PathInterner!();
        ChangedFile!();
        FileId!();
    };
}

macro_rules! Vfs {
    () => {
        deps!();
        # [doc = " Storage for all file changes and the file id to path mapping."] # [doc = ""] # [doc = " For more information see the [crate-level](crate) documentation."] # [derive (Default)] pub struct Vfs { interner : PathInterner , data : Vec < FileState > , changes : IndexMap < FileId , ChangedFile , BuildHasherDefault < FxHasher > > , }
    };
}

Vfs!()