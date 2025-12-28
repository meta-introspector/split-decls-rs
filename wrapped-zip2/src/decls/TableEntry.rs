macro_rules! TableEntry {
    () => {
        # [derive (Default , Clone , Copy)] pub struct TableEntry { # [doc = " Wide enough to fit the max symbol nbr."] pub sym : u16 , # [doc = " 0 means no symbol."] pub len : u8 , }
    };
}

TableEntry!();