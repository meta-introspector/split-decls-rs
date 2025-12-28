macro_rules! impl_246 {
    () => {
        # [cfg (any (feature = "parsing" , feature = "printing"))] impl Member { pub (crate) fn is_named (& self) -> bool { match self { Member :: Named (_) => true , Member :: Unnamed (_) => false , } } }
    };
}

impl_246!();