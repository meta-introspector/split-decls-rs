macro_rules! deps {
    () => {
        Magic!();
    };
}

macro_rules! ExtraFieldMagic {
    () => {
        deps!();
        # [doc = " Similar to [`Magic`], but used for extra field tags as per section 4.5.3 of APPNOTE.TXT."] # [derive (Copy , Clone , Debug , PartialOrd , Ord , PartialEq , Eq , Hash)] # [repr (transparent)] pub (crate) struct ExtraFieldMagic (u16) ;
    };
}

ExtraFieldMagic!();