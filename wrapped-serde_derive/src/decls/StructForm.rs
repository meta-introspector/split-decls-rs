macro_rules! StructForm {
    () => {
        enum StructForm < 'a > { Struct , # [doc = " Contains a variant name"] ExternallyTagged (& 'a syn :: Ident) , # [doc = " Contains a variant name"] InternallyTagged (& 'a syn :: Ident) , # [doc = " Contains a variant name"] Untagged (& 'a syn :: Ident) , }
    };
}

StructForm!()