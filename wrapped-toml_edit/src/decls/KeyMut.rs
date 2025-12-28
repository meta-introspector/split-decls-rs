macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! KeyMut {
    () => {
        deps!();
        # [doc = " A mutable reference to a [`Key`]'s formatting"] # [derive (Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct KeyMut < 'k > { key : & 'k mut Key , }
    };
}

KeyMut!()