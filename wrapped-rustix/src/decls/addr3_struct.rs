macro_rules! addr3_struct {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Copy , Clone , Default)] # [non_exhaustive] pub struct addr3_struct { pub addr3 : u64 , # [doc (hidden)] pub __pad2 : [u64 ; 1] , }
    };
}

addr3_struct!()