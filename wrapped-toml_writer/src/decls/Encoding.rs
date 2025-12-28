macro_rules! Encoding {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [repr (u8)] # [allow (clippy :: enum_variant_names)] enum Encoding { LiteralString , BasicString , MlLiteralString , MlBasicString , }
    };
}

Encoding!()