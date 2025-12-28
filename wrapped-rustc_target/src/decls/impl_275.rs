macro_rules! deps {
    () => {
        Uniform!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl From < Reg > for Uniform { fn from (unit : Reg) -> Uniform { Uniform { unit , total : unit . size , is_consecutive : false } } }
    };
}

impl_275!()