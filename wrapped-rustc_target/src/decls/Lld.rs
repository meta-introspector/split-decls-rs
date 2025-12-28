macro_rules! Lld {
    () => {
        # [doc = " Linker is LLD."] # [derive (Clone , Copy , Debug , Eq , Ord , PartialEq , PartialOrd)] pub enum Lld { Yes , No , }
    };
}

Lld!();