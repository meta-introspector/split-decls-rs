macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl From < std :: process :: Command > for Command { fn from (cmd : std :: process :: Command) -> Self { Self :: from_std (cmd) } }
    };
}

impl_43!();