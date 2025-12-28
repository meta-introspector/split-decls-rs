macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! alloc_support {
    () => {
        deps!();
        # [cfg (feature = "alloc")] mod alloc_support { use super :: * ; use crate :: std :: boxed :: Box ; impl_value_forward ! ({ impl < T : Value + ? Sized > Value for Box < T > } => x => { ** x }) ; }
    };
}

alloc_support!();