macro_rules! deps {
    () => {
        FileAstId!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [doc = " Traits are manually implemented because `derive` adds redundant bounds."] impl < N > Clone for FileAstId < N > { # [inline] fn clone (& self) -> FileAstId < N > { * self } }
    };
}

impl_15!()