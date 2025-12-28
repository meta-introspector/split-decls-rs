macro_rules! scan_expr {
    () => {
        # [cfg (all (feature = "parsing" , feature = "derive" , not (feature = "full")))] mod scan_expr ;
    };
}

scan_expr!();