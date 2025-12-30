// Generated macro for impl_59 (impl)
macro_rules! Depcrate_errorimpl_59 {
() => {
// Module: crate::error
// Provides: {"impl_59"}
// Dependencies: {}
impl < I : Stream , E : ParserError < I > > ParserError < I > for ErrMode < E > { type Inner = E ; # [inline (always)] fn from_input (input : & I) -> Self { ErrMode :: Backtrack (E :: from_input (input)) } # [inline (always)] fn assert (input : & I , message : & 'static str) -> Self where I : core :: fmt :: Debug , { ErrMode :: Cut (E :: assert (input , message)) } # [inline (always)] fn incomplete (_input : & I , needed : Needed) -> Self { ErrMode :: Incomplete (needed) } # [inline] fn append (self , input : & I , token_start : & < I as Stream > :: Checkpoint) -> Self { match self { ErrMode :: Backtrack (e) => ErrMode :: Backtrack (e . append (input , token_start)) , e => e , } } fn or (self , other : Self) -> Self { match (self , other) { (ErrMode :: Backtrack (e) , ErrMode :: Backtrack (o)) => ErrMode :: Backtrack (e . or (o)) , (ErrMode :: Incomplete (e) , _) | (_ , ErrMode :: Incomplete (e)) => ErrMode :: Incomplete (e) , (ErrMode :: Cut (e) , _) | (_ , ErrMode :: Cut (e)) => ErrMode :: Cut (e) , } } # [inline (always)] fn is_backtrack (& self) -> bool { matches ! (self , ErrMode :: Backtrack (_)) } # [inline (always)] fn into_inner (self) -> Result < Self :: Inner , Self > { match self { ErrMode :: Backtrack (e) | ErrMode :: Cut (e) => Ok (e) , err @ ErrMode :: Incomplete (_) => Err (err) , } } # [inline (always)] fn is_incomplete (& self) -> bool { matches ! (self , ErrMode :: Incomplete (_)) } # [inline (always)] fn needed (& self) -> Option < Needed > { match self { ErrMode :: Incomplete (needed) => Some (* needed) , _ => None , } } }
};
}
