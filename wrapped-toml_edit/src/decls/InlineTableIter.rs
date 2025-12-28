macro_rules! deps {
    () => {
        Item!();
        Value!();
    };
}

macro_rules! InlineTableIter {
    () => {
        deps!();
        # [doc = " An iterator type over [`InlineTable`]'s [`Key`]/[`Value`] pairs"] pub type InlineTableIter < 'a > = Box < dyn Iterator < Item = (& 'a str , & 'a Value) > + 'a > ;
    };
}

InlineTableIter!()