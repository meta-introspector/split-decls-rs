macro_rules! deps {
    () => {
        Item!();
        Value!();
    };
}

macro_rules! InlineTableIntoIter {
    () => {
        deps!();
        # [doc = " An owned iterator type over an [`InlineTable`]'s [`Key`]/[`Value`] pairs"] pub type InlineTableIntoIter = Box < dyn Iterator < Item = (String , Value) > > ;
    };
}

InlineTableIntoIter!()