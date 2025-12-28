macro_rules! deps {
    () => {
        Crate!();
    };
}

macro_rules! external_crates {
    () => {
        deps!();
        # [doc = " Try to find a crate with the given name."] pub fn external_crates () -> Vec < Crate > { with (| cx | cx . external_crates ()) }
    };
}

external_crates!()