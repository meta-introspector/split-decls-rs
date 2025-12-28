macro_rules! style {
    () => {
        # [cfg (feature = "display")] mod style ;
    };
}

style!();