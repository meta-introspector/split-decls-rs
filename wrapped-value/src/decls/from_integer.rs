macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! from_integer {
    () => {
        deps!();
        macro_rules ! from_integer { ($ ($ ty : ident) ,*) => { $ (impl From <$ ty > for ConstValue { # [inline] fn from (n : $ ty) -> Self { ConstValue :: Number (n . into ()) } }) * } ; }
    };
}

from_integer!();