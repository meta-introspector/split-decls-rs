macro_rules! deps {
    () => {
        Flavor!();
    };
}

macro_rules! X86Options {
    () => {
        deps!();
        pub (crate) struct X86Options { pub flavor : Flavor , pub regparm : Option < u32 > , pub reg_struct_return : bool , }
    };
}

X86Options!()