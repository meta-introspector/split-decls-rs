macro_rules! OpaqueTypeStorageEntries {
    () => {
        pub trait OpaqueTypeStorageEntries : Debug + Copy + Default { # [doc = " Whether the number of opaques has changed in a way that necessitates"] # [doc = " reevaluating a goal. For now, this is only when the number of non-duplicated"] # [doc = " entries changed."] fn needs_reevaluation (self , canonicalized : usize) -> bool ; }
    };
}

OpaqueTypeStorageEntries!();