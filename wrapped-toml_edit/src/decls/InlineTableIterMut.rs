macro_rules! deps {
    () => {
        KeyMut!();
        Item!();
        Value!();
    };
}

macro_rules! InlineTableIterMut {
    () => {
        deps!();
        # [doc = " A mutable iterator type over [`InlineTable`]'s [`Key`]/[`Value`] pairs"] pub type InlineTableIterMut < 'a > = Box < dyn Iterator < Item = (KeyMut < 'a > , & 'a mut Value) > + 'a > ;
    };
}

InlineTableIterMut!();