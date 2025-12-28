macro_rules! deps {
    () => {
        RawString!();
        Item!();
        Table!();
    };
}

macro_rules! DocumentMut {
    () => {
        deps!();
        # [doc = " The editable root TOML [`Table`], containing [`Key`][crate::Key]/[`Value`][crate::Value] pairs and all other logic [`Table`]s"] # [derive (Debug , Clone)] pub struct DocumentMut { pub (crate) root : Item , pub (crate) trailing : RawString , }
    };
}

DocumentMut!()