macro_rules! deps {
    () => {
        FromRawReprsError!();
        RawRepr!();
        FromAttrsError!();
        AlignRepr!();
        Repr!();
        CompoundRepr!();
        PrimitiveRepr!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < Prim , Packed > Repr < Prim , Packed > { fn from_attrs_inner (attrs : & [Attribute]) -> Result < Repr < Prim , Packed > , Spanned < FromAttrsError > > where Prim : With < PrimitiveRepr > , Packed : With < NonZeroU32 > , { let raw_reprs = RawRepr :: from_attrs (attrs) . map_err (Spanned :: from) ? ; let transparent = { let mut transparents = raw_reprs . iter () . filter_map (| Spanned { t , span } | match t { RawRepr :: Transparent => Some (span) , _ => None , }) ; let first = transparents . next () ; let second = transparents . next () ; match (first , second) { (None , None) => None , (Some (span) , None) => Some (* span) , (Some (_) , Some (second)) => { return Err (Spanned :: new (FromAttrsError :: FromRawReprs (FromRawReprsError :: Conflict) , * second ,)) } (None , Some (_)) => unreachable ! () , } } ; let compound : Option < Spanned < CompoundRepr < Prim > > > = try_from_raw_reprs (raw_reprs . iter ()) . map_err (Spanned :: from) ? ; let align : Option < Spanned < AlignRepr < Packed > > > = try_from_raw_reprs (raw_reprs . iter ()) . map_err (Spanned :: from) ? ; if let Some (span) = transparent { if compound . is_some () || align . is_some () { return Err (Spanned :: new (FromRawReprsError :: Conflict . into () , span)) ; } Ok (Repr :: Transparent (span)) } else { Ok (Repr :: Compound (compound . unwrap_or (Spanned :: new (CompoundRepr :: Rust , Span :: call_site ())) , align ,)) } } }
    };
}

impl_52!()