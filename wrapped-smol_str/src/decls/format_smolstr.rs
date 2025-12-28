macro_rules! deps {
    () => {
        SmolStrBuilder!();
        SmolStr!();
    };
}

macro_rules! format_smolstr {
    () => {
        deps!();
        # [doc = " Formats arguments to a [`SmolStr`], potentially without allocating."] # [doc = ""] # [doc = " See [`alloc::format!`] or [`format_args!`] for syntax documentation."] # [macro_export] macro_rules ! format_smolstr { ($ ($ tt : tt) *) => { { let mut w = $ crate :: SmolStrBuilder :: new () ; :: core :: fmt :: Write :: write_fmt (& mut w , format_args ! ($ ($ tt) *)) . expect ("a formatting trait implementation returned an error") ; w . finish () } } ; }
    };
}

format_smolstr!();