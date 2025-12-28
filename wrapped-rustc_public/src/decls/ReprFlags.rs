macro_rules! ReprFlags {
    () => {
        # [non_exhaustive] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Serialize)] pub struct ReprFlags { pub is_simd : bool , pub is_c : bool , pub is_transparent : bool , pub is_linear : bool , }
    };
}

ReprFlags!()