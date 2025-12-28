macro_rules! deps {
    () => {
        AssocItem!();
    };
}

macro_rules! AssocItems {
    () => {
        deps!();
        # [doc = " A list of associated items."] pub type AssocItems = Vec < AssocItem > ;
    };
}

AssocItems!()