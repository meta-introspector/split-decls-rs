macro_rules! deps {
    () => {
        DeValue!();
    };
}

macro_rules! DeArray {
    () => {
        deps!();
        # [doc = " Type representing a TOML array, payload of the `DeValue::Array` variant"] # [derive (Clone)] pub struct DeArray < 'i > { items : Vec < Spanned < DeValue < 'i > > > , array_of_tables : bool , }
    };
}

DeArray!()