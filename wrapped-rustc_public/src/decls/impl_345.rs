macro_rules! deps {
    () => {
        PolyFnSig!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl FnDef { pub fn body (& self) -> Option < Body > { with (| ctx | ctx . has_body (self . 0) . then (| | ctx . mir_body (self . 0))) } pub fn has_body (& self) -> bool { with (| ctx | ctx . has_body (self . 0)) } # [doc = " Get the information of the intrinsic if this function is a definition of one."] pub fn as_intrinsic (& self) -> Option < IntrinsicDef > { with (| cx | cx . intrinsic (self . def_id ())) } # [doc = " Check if the function is an intrinsic."] # [inline] pub fn is_intrinsic (& self) -> bool { self . as_intrinsic () . is_some () } # [doc = " Get the function signature for this function definition."] pub fn fn_sig (& self) -> PolyFnSig { let kind = self . ty () . kind () ; kind . fn_sig () . unwrap () } }
    };
}

impl_345!()