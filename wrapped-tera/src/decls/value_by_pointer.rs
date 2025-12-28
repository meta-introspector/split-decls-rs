macro_rules! deps {
    () => {
        Val!();
    };
}

macro_rules! value_by_pointer {
    () => {
        deps!();
        # [doc = " Gets a value within a value by pointer, keeping lifetime"] # [inline] pub fn value_by_pointer < 'a > (pointer : & str , val : & Val < 'a >) -> Option < Val < 'a > > { match * val { Cow :: Borrowed (r) => dotted_pointer (r , pointer) . map (Cow :: Borrowed) , Cow :: Owned (ref r) => dotted_pointer (r , pointer) . map (| found | Cow :: Owned (found . clone ())) , } }
    };
}

value_by_pointer!()