macro_rules! deps {
    () => {
        TokenBuffer!();
        Entry!();
        Group!();
        End!();
        Cursor!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl TokenBuffer { fn recursive_new (entries : & mut Vec < Entry > , stream : TokenStream) { for tt in stream { match tt { TokenTree :: Ident (ident) => entries . push (Entry :: Ident (ident)) , TokenTree :: Punct (punct) => entries . push (Entry :: Punct (punct)) , TokenTree :: Literal (literal) => entries . push (Entry :: Literal (literal)) , TokenTree :: Group (group) => { let group_start_index = entries . len () ; entries . push (Entry :: End (0 , 0)) ; Self :: recursive_new (entries , group . stream ()) ; let group_end_index = entries . len () ; let group_offset = group_end_index - group_start_index ; entries . push (Entry :: End (- (group_end_index as isize) , - (group_offset as isize) ,)) ; entries [group_start_index] = Entry :: Group (group , group_offset) ; } } } } # [doc = " Creates a `TokenBuffer` containing all the tokens from the input"] # [doc = " `proc_macro::TokenStream`."] # [cfg (feature = "proc-macro")] # [cfg_attr (docsrs , doc (cfg (feature = "proc-macro")))] pub fn new (stream : proc_macro :: TokenStream) -> Self { Self :: new2 (stream . into ()) } # [doc = " Creates a `TokenBuffer` containing all the tokens from the input"] # [doc = " `proc_macro2::TokenStream`."] pub fn new2 (stream : TokenStream) -> Self { let mut entries = Vec :: new () ; Self :: recursive_new (& mut entries , stream) ; entries . push (Entry :: End (- (entries . len () as isize) , 0)) ; Self { entries : entries . into_boxed_slice () , } } # [doc = " Creates a cursor referencing the first token in the buffer and able to"] # [doc = " traverse until the end of the buffer."] pub fn begin (& self) -> Cursor { let ptr = self . entries . as_ptr () ; unsafe { Cursor :: create (ptr , ptr . add (self . entries . len () - 1)) } } }
    };
}

impl_92!();