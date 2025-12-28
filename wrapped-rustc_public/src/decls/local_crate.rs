macro_rules! deps {
    () => {
        Crate!();
    };
}

macro_rules! local_crate {
    () => {
        deps!();
        # [doc = " Access to the local crate."] pub fn local_crate () -> Crate { with (| cx | cx . local_crate ()) }
    };
}

local_crate!()