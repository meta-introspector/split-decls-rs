macro_rules! deps {
    () => {
        SyntaxContext!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl SyntaxContext { # [inline] pub fn is_root (self) -> bool { (SyntaxContext :: MAX_ID - Edition :: LATEST as u32) <= self . into_u32 () && self . into_u32 () <= (SyntaxContext :: MAX_ID - Edition :: Edition2015 as u32) } # [inline] pub fn remove_root_edition (& mut self) { if self . is_root () { * self = Self :: root (Edition :: Edition2015) ; } } # [doc = " The root context, which is the parent of all other contexts. All [`FileId`]s have this context."] # [inline] pub const fn root (edition : Edition) -> Self { let edition = edition as u32 ; unsafe { SyntaxContext :: from_u32 (SyntaxContext :: MAX_ID - edition) } } }
    };
}

impl_56!()