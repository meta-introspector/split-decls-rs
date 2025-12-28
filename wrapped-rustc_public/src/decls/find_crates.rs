macro_rules! deps {
    () => {
        Crate!();
    };
}

macro_rules! find_crates {
    () => {
        deps!();
        # [doc = " Try to find a crate or crates if multiple crates exist from given name."] pub fn find_crates (name : & str) -> Vec < Crate > { with (| cx | cx . find_crates (name)) }
    };
}

find_crates!();