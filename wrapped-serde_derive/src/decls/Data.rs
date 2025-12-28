macro_rules! deps {
    () => {
        Variant!();
        Field!();
        Style!();
    };
}

macro_rules! Data {
    () => {
        deps!();
        # [doc = " The fields of a struct or enum."] # [doc = ""] # [doc = " Analogous to `syn::Data`."] pub enum Data < 'a > { Enum (Vec < Variant < 'a > >) , Struct (Style , Vec < Field < 'a > >) , }
    };
}

Data!()