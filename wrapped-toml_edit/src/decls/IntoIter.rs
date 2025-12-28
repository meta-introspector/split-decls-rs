macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! IntoIter {
    () => {
        deps!();
        # [doc = " An owned iterator type over [`Table`]'s [`Key`]/[`Item`] pairs"] pub type IntoIter = Box < dyn Iterator < Item = (String , Item) > > ;
    };
}

IntoIter!()