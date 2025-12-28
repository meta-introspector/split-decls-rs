macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " An iterator type over [`Table`]'s [`Key`]/[`Item`] pairs"] pub type Iter < 'a > = Box < dyn Iterator < Item = (& 'a str , & 'a Item) > + 'a > ;
    };
}

Iter!()