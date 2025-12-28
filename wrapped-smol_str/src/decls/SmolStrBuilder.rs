macro_rules! deps {
    () => {
        SmolStr!();
        SmolStrBuilderRepr!();
    };
}

macro_rules! SmolStrBuilder {
    () => {
        deps!();
        # [doc = " A builder that can be used to efficiently build a [`SmolStr`]."] # [doc = ""] # [doc = " This won't allocate if the final string fits into the inline buffer."] # [derive (Clone , Default , Debug , PartialEq , Eq)] pub struct SmolStrBuilder (SmolStrBuilderRepr) ;
    };
}

SmolStrBuilder!()