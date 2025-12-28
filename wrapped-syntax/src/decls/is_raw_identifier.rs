macro_rules! is_raw_identifier {
    () => {
        # [inline] pub fn is_raw_identifier (name : & str , edition : parser :: Edition) -> bool { let is_keyword = SyntaxKind :: from_keyword (name , edition) . is_some () ; is_keyword && ! matches ! (name , "self" | "crate" | "super" | "Self") }
    };
}

is_raw_identifier!()