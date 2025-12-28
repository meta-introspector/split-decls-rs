macro_rules! U8SliceExt {
    () => {
        pub trait U8SliceExt { fn first_u32 (& self) -> Option < u32 > ; fn last_u32 (& self) -> Option < u32 > ; fn first_u64 (& self) -> Option < u64 > ; fn last_u64 (& self) -> Option < u64 > ; }
    };
}

U8SliceExt!();