macro_rules! Position {
    () => {
        # [derive (Copy , Clone , Eq , PartialEq)] pub (crate) enum Position { Major , Minor , Patch , Pre , Build , }
    };
}

Position!();