macro_rules! Code {
    () => {
        # [derive (Debug , Default , Clone , Copy , PartialEq , Eq)] pub (crate) struct Code { # [doc = " operation, extra bits, table bits"] pub op : u8 , # [doc = " bits in this part of the code"] pub bits : u8 , # [doc = " offset in table or code value"] pub val : u16 , }
    };
}

Code!();