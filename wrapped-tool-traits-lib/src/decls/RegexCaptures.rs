macro_rules! RegexCaptures {
    () => {
        pub trait RegexCaptures { fn get (& self , i : usize) -> Option < & str > ; fn len (& self) -> usize ; }
    };
}

RegexCaptures!();