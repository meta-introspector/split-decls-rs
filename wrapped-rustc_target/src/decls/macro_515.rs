macro_rules! macro_515 {
    () => {
        crate :: target_spec_enum ! { pub enum FramePointer { # [doc = " Forces the machine code generator to always preserve the frame pointers."] Always = "always" , # [doc = " Forces the machine code generator to preserve the frame pointers except for the leaf"] # [doc = " functions (i.e. those that don't call other functions)."] NonLeaf = "non-leaf" , # [doc = " Allows the machine code generator to omit the frame pointers."] # [doc = ""] # [doc = " This option does not guarantee that the frame pointers will be omitted."] MayOmit = "may-omit" , } parse_error_type = "frame pointer" ; }
    };
}

macro_515!();