macro_rules! ArrayLen {
    () => {
        # [derive (Default , Eq , PartialEq , Ord , PartialOrd , Copy , Clone)] pub struct ArrayLen (usize) ;
    };
}

ArrayLen!();