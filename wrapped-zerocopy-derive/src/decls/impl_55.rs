macro_rules! deps {
    () => {
        RawRepr!();
        UnrecognizedReprError!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl RawRepr { fn from_attrs (attrs : & [Attribute] ,) -> Result < Vec < Spanned < RawRepr > > , Spanned < UnrecognizedReprError > > { let mut reprs = Vec :: new () ; for attr in attrs { if attr . path () . is_ident ("doc") { continue ; } if let Meta :: List (ref meta_list) = attr . meta { if meta_list . path . is_ident ("repr") { let parsed : Punctuated < Meta , Comma > = match meta_list . parse_args_with (Punctuated :: parse_terminated) { Ok (parsed) => parsed , Err (_) => { return Err (Spanned :: new (UnrecognizedReprError , meta_list . tokens . span () ,)) } } ; for meta in parsed { let s = meta . span () ; reprs . push (RawRepr :: from_meta (& meta) . map (| r | Spanned :: new (r , s)) . map_err (| e | Spanned :: new (e , s)) ? ,) ; } } } } Ok (reprs) } fn from_meta (meta : & Meta) -> Result < RawRepr , UnrecognizedReprError > { let (path , list) = match meta { Meta :: Path (path) => (path , None) , Meta :: List (list) => (& list . path , Some (list)) , _ => return Err (UnrecognizedReprError) , } ; let ident = path . get_ident () . ok_or (UnrecognizedReprError) ? ; let parse_nzu64 = | list : & MetaList | { list . parse_args :: < LitInt > () . and_then (| int | int . base10_parse :: < NonZeroU32 > ()) . map_err (| _ | UnrecognizedReprError) . and_then (| nz | { if nz . get () . is_power_of_two () { Ok (nz) } else { Err (UnrecognizedReprError) } }) } ; use RawRepr :: * ; Ok (match (ident . to_string () . as_str () , list) { ("u8" , None) => U8 , ("u16" , None) => U16 , ("u32" , None) => U32 , ("u64" , None) => U64 , ("u128" , None) => U128 , ("usize" , None) => Usize , ("i8" , None) => I8 , ("i16" , None) => I16 , ("i32" , None) => I32 , ("i64" , None) => I64 , ("i128" , None) => I128 , ("isize" , None) => Isize , ("C" , None) => C , ("transparent" , None) => Transparent , ("Rust" , None) => Rust , ("packed" , None) => Packed , ("packed" , Some (list)) => PackedN (parse_nzu64 (list) ?) , ("align" , Some (list)) => Align (parse_nzu64 (list) ?) , _ => return Err (UnrecognizedReprError) , }) } }
    };
}

impl_55!()