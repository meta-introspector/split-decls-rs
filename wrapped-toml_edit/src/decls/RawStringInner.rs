macro_rules! RawStringInner {
    () => {
        # [derive (PartialEq , Eq , Clone , Hash)] enum RawStringInner { Empty , Explicit (String) , Spanned (std :: ops :: Range < usize >) , }
    };
}

RawStringInner!()