macro_rules! TupleForm {
    () => {
        enum TupleForm < 'a > { Tuple , # [doc = " Contains a variant name"] ExternallyTagged (& 'a syn :: Ident) , # [doc = " Contains a variant name"] Untagged (& 'a syn :: Ident) , }
    };
}

TupleForm!()