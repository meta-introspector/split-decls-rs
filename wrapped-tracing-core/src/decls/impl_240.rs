macro_rules! deps {
    () => {
        Id!();
        CurrentInner!();
        Current!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl From < Current > for Option < Id > { fn from (cur : Current) -> Self { match cur . inner { CurrentInner :: Current { id , .. } => Some (id) , _ => None , } } }
    };
}

impl_240!();