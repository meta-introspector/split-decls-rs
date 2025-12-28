macro_rules! deps {
    () => {
        Symbol!();
        Ctxt!();
    };
}

macro_rules! VecAttr {
    () => {
        deps!();
        pub (crate) struct VecAttr < 'c , T > { cx : & 'c Ctxt , name : Symbol , first_dup_tokens : TokenStream , values : Vec < T > , }
    };
}

VecAttr!()