macro_rules! deps {
    () => {
        Variant!();
        Field!();
    };
}

macro_rules! Identifier {
    () => {
        deps!();
        # [doc = " Whether this enum represents the fields of a struct or the variants of an"] # [doc = " enum."] # [derive (Copy , Clone)] pub enum Identifier { # [doc = " It does not."] No , # [doc = " This enum represents the fields of a struct. All of the variants must be"] # [doc = " unit variants, except possibly one which is annotated with"] # [doc = " `#[serde(other)]` and is a newtype variant."] Field , # [doc = " This enum represents the variants of an enum. All of the variants must"] # [doc = " be unit variants."] Variant , }
    };
}

Identifier!();