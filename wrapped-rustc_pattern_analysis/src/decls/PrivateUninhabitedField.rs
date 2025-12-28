macro_rules! PrivateUninhabitedField {
    () => {
        # [doc = " `bool` newtype that indicates whether this is a privately uninhabited field that we should skip"] # [doc = " during analysis."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct PrivateUninhabitedField (pub bool) ;
    };
}

PrivateUninhabitedField!();