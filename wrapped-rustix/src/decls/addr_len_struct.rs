macro_rules! addr_len_struct {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Copy , Clone)] # [non_exhaustive] pub struct addr_len_struct { pub addr_len : u16 , # [doc (hidden)] pub __pad3 : [u16 ; 1] , }
    };
}

addr_len_struct!()