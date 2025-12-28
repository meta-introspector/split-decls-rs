macro_rules! deps {
    () => {
        SpecialCodeIndex!();
    };
}

macro_rules! SpecialCodes {
    () => {
        deps!();
        # [doc = " An array indexed by [`SpecialCodeIndex`] indicating the current values of"] # [doc = " various special control codes."] # [repr (transparent)] # [derive (Clone)] pub struct SpecialCodes (pub (crate) [c :: cc_t ; c :: NCCS as usize]) ;
    };
}

SpecialCodes!()