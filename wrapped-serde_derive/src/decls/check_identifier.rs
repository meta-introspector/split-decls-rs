macro_rules! deps {
    () => {
        Container!();
        Variant!();
        Data!();
        Field!();
        TagType!();
        Ctxt!();
        Identifier!();
        Style!();
    };
}

macro_rules! check_identifier {
    () => {
        deps!();
        fn check_identifier (cx : & Ctxt , cont : & Container) { let variants = match & cont . data { Data :: Enum (variants) => variants , Data :: Struct (_ , _) => return , } ; for (i , variant) in variants . iter () . enumerate () { match (variant . style , cont . attrs . identifier () , variant . attrs . other () , cont . attrs . tag () ,) { (_ , Identifier :: Variant , true , _) => { cx . error_spanned_by (variant . original , "#[serde(other)] may not be used on a variant identifier" ,) ; } (_ , Identifier :: No , true , & TagType :: None) => { cx . error_spanned_by (variant . original , "#[serde(other)] cannot appear on untagged enum" ,) ; } (Style :: Unit , Identifier :: Field , true , _) | (Style :: Unit , Identifier :: No , true , _) => { if i < variants . len () - 1 { cx . error_spanned_by (variant . original , "#[serde(other)] must be on the last variant" ,) ; } } (_ , Identifier :: Field , true , _) | (_ , Identifier :: No , true , _) => { cx . error_spanned_by (variant . original , "#[serde(other)] must be on a unit variant" ,) ; } (_ , Identifier :: No , false , _) => { } (Style :: Unit , _ , false , _) => { } (Style :: Newtype , Identifier :: Field , false , _) => { if i < variants . len () - 1 { cx . error_spanned_by (variant . original , format ! ("`{}` must be the last variant" , variant . ident) ,) ; } } (_ , Identifier :: Field , false , _) => { cx . error_spanned_by (variant . original , "#[serde(field_identifier)] may only contain unit variants" ,) ; } (_ , Identifier :: Variant , false , _) => { cx . error_spanned_by (variant . original , "#[serde(variant_identifier)] may only contain unit variants" ,) ; } } } }
    };
}

check_identifier!()