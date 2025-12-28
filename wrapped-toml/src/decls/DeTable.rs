macro_rules! deps {
    () => {
        DeValue!();
        Map!();
        DeString!();
    };
}

macro_rules! DeTable {
    () => {
        deps!();
        # [doc = " Type representing a TOML table, payload of the `Value::Table` variant."] # [doc = ""] # [doc = " By default it entries are stored in"] # [doc = " [lexicographic order](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str)"] # [doc = " of the keys. Enable the `preserve_order` feature to store entries in the order they appear in"] # [doc = " the source file."] pub type DeTable < 'i > = Map < Spanned < DeString < 'i > > , Spanned < DeValue < 'i > > > ;
    };
}

DeTable!()