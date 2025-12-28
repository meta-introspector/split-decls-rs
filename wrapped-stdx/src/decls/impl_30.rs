macro_rules! deps {
    () => {
        JodChild!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl JodChild { pub fn spawn (mut command : Command) -> sio :: Result < Self > { command . spawn () . map (Self) } # [must_use] # [cfg (not (target_arch = "wasm32"))] pub fn into_inner (self) -> std :: process :: Child { unsafe { std :: mem :: transmute :: < Self , std :: process :: Child > (self) } } }
    };
}

impl_30!()