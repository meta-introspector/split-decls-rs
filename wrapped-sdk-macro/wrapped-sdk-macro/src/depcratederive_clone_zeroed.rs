// Generated macro for derive_clone_zeroed (function)
macro_rules! Depcratederive_clone_zeroed {
() => {
// Module: crate
// Provides: {"derive_clone_zeroed"}
// Dependencies: {}
# [proc_macro_derive (CloneZeroed)] pub fn derive_clone_zeroed (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { match parse_macro_input ! (input as syn :: Item) { syn :: Item :: Struct (item_struct) => { let clone_statements = match item_struct . fields { syn :: Fields :: Named (ref fields) => fields . named . iter () . map (| f | { let name = & f . ident ; quote ! { core :: ptr :: addr_of_mut ! ((* ptr) .# name) . write (self .# name . clone ()) ; } }) , _ => unimplemented ! () , } ; let name = & item_struct . ident ; quote ! { impl Clone for # name { fn clone (& self) -> Self { let mut value = core :: mem :: MaybeUninit ::< Self >:: uninit () ; unsafe { core :: ptr :: write_bytes (& mut value , 0 , 1) ; let ptr = value . as_mut_ptr () ; # (# clone_statements) * value . assume_init () } } } } } _ => unimplemented ! () , } . into () }
};
}
