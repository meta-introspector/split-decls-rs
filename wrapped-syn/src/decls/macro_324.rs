macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_324 {
    () => {
        deps!();
        ast_struct ! { # [doc = " Precise capturing bound: the 'use&lt;&hellip;&gt;' in `impl Trait +"] # [doc = " use<'a, T>`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PreciseCapture # full { pub use_token : Token ! [use] , pub lt_token : Token ! [<] , pub params : Punctuated < CapturedParam , Token ! [,] >, pub gt_token : Token ! [>] , } }
    };
}

macro_324!()