// Generated macro for impl_121 (impl)
macro_rules! Depcrate_errorimpl_121 {
() => {
// Module: crate::error
// Provides: {"impl_121"}
// Dependencies: {}
# [cfg (feature = "std")] impl < I , C > ParserError < I > for TreeError < I , C > where I : Stream + Clone , { type Inner = Self ; fn from_input (input : & I) -> Self { TreeError :: Base (TreeErrorBase { input : input . clone () , cause : None , }) } fn append (self , input : & I , token_start : & < I as Stream > :: Checkpoint) -> Self { let mut input = input . clone () ; input . reset (token_start) ; let frame = TreeErrorFrame :: Kind (TreeErrorBase { input , cause : None }) ; self . append_frame (frame) } fn or (self , other : Self) -> Self { match (self , other) { (TreeError :: Alt (mut first) , TreeError :: Alt (second)) => { first . extend (second) ; TreeError :: Alt (first) } (TreeError :: Alt (mut alt) , new) | (new , TreeError :: Alt (mut alt)) => { alt . push (new) ; TreeError :: Alt (alt) } (first , second) => TreeError :: Alt (vec ! [first , second]) , } } # [inline (always)] fn into_inner (self) -> Result < Self :: Inner , Self > { Ok (self) } }
};
}
