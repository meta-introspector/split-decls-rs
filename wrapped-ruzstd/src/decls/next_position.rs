macro_rules! next_position {
    () => {
        # [doc = " Calculate the position of the next entry of the table given the current"] # [doc = " position and size of the table."] fn next_position (mut p : usize , table_size : usize) -> usize { p += (table_size >> 1) + (table_size >> 3) + 3 ; p &= table_size - 1 ; p }
    };
}

next_position!();