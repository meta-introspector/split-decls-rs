macro_rules! deps {
    () => {
        TokenTree!();
    };
}

macro_rules! Cursor {
    () => {
        deps!();
        pub struct Cursor < 'a , Span > { buffer : & 'a [TokenTree < Span >] , index : usize , subtrees_stack : Vec < usize > , }
    };
}

Cursor!()