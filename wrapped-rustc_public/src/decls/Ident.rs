macro_rules! deps {
    () => {
        Opaque!();
    };
}

macro_rules! Ident {
    () => {
        deps!();
        type Ident = Opaque ;
    };
}

Ident!()