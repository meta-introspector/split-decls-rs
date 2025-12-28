macro_rules! deps {
    () => {
        ExternAbiWrapper!();
        ABI!();
    };
}

macro_rules! impl_444 {
    () => {
        deps!();
        impl FromStr for ExternAbiWrapper { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { rustc_abi :: ExternAbi :: from_str (s) . map (Self) . map_err (| _ | format ! ("{s} is not a valid extern ABI")) } }
    };
}

impl_444!();