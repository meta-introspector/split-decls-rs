macro_rules! deps {
    () => {
        Decor!();
        RawString!();
        KeyValuePairs!();
        Value!();
    };
}

macro_rules! InlineTable {
    () => {
        deps!();
        # [doc = " A TOML [`Value`] that contains a collection of [`Key`]/[`Value`] pairs"] # [derive (Debug , Default , Clone)] pub struct InlineTable { preamble : RawString , pub (crate) implicit : bool , decor : Decor , pub (crate) span : Option < std :: ops :: Range < usize > > , dotted : bool , pub (crate) items : KeyValuePairs , }
    };
}

InlineTable!()