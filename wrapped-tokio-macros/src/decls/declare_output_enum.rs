macro_rules! declare_output_enum {
    () => {
        pub (crate) fn declare_output_enum (input : TokenStream) -> TokenStream { let branches = match input . into_iter () . next () { Some (TokenTree :: Group (group)) => group . stream () . into_iter () . count () , _ => panic ! ("unexpected macro input") , } ; let variants = (0 .. branches) . map (| num | Ident :: new (& format ! ("_{num}") , Span :: call_site ())) . collect :: < Vec < _ > > () ; let mask = Ident :: new (if branches <= 8 { "u8" } else if branches <= 16 { "u16" } else if branches <= 32 { "u32" } else if branches <= 64 { "u64" } else { panic ! ("up to 64 branches supported") ; } , Span :: call_site () ,) ; TokenStream :: from (quote ! { pub (super) enum Out <# (# variants) ,*> { # (# variants (# variants) ,) * Disabled , } pub (super) type Mask = # mask ; }) }
    };
}

declare_output_enum!()