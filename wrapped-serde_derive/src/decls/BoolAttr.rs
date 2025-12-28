macro_rules! deps {
    () => {
        Attr!();
    };
}

macro_rules! BoolAttr {
    () => {
        deps!();
        struct BoolAttr < 'c > (Attr < 'c , () >) ;
    };
}

BoolAttr!();