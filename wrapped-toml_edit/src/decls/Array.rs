macro_rules! deps {
    () => {
        RawString!();
        Item!();
        Value!();
        Decor!();
    };
}

macro_rules! Array {
    () => {
        deps!();
        # [doc = " A TOML [`Value`] that contains a sequence of [`Value`]s"] # [derive (Debug , Default , Clone)] pub struct Array { trailing : RawString , trailing_comma : bool , decor : Decor , pub (crate) span : Option < std :: ops :: Range < usize > > , pub (crate) values : Vec < Item > , }
    };
}

Array!();