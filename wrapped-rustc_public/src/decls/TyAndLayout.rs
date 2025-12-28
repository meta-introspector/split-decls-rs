macro_rules! deps {
    () => {
        Ty!();
        Layout!();
    };
}

macro_rules! TyAndLayout {
    () => {
        deps!();
        # [doc = " The layout of a type, alongside the type itself."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub struct TyAndLayout { pub ty : Ty , pub layout : Layout , }
    };
}

TyAndLayout!();