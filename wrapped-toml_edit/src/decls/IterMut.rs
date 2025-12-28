macro_rules! deps {
    () => {
        Item!();
        KeyMut!();
    };
}

macro_rules! IterMut {
    () => {
        deps!();
        # [doc = " A mutable iterator type over [`Table`]'s [`Key`]/[`Item`] pairs"] pub type IterMut < 'a > = Box < dyn Iterator < Item = (KeyMut < 'a > , & 'a mut Item) > + 'a > ;
    };
}

IterMut!()