macro_rules! cmd_op_struct {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Copy , Clone)] # [non_exhaustive] pub struct cmd_op_struct { pub cmd_op : u32 , # [doc (hidden)] pub __pad1 : u32 , }
    };
}

cmd_op_struct!();