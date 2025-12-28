macro_rules! Expected {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug)] # [non_exhaustive] pub enum Expected { Literal (& 'static str) , Description (& 'static str) , }
    };
}

Expected!();