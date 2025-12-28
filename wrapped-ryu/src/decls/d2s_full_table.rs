macro_rules! d2s_full_table {
    () => {
        # [cfg (not (feature = "small"))] mod d2s_full_table ;
    };
}

d2s_full_table!()