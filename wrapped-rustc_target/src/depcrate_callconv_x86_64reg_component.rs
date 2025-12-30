// Generated macro for reg_component (function)
macro_rules! Depcrate_callconv_x86_64reg_component {
() => {
// Module: crate::callconv::x86_64
// Provides: {"reg_component"}
// Dependencies: {}
fn reg_component (cls : & [Option < Class >] , i : & mut usize , size : Size) -> Option < Reg > { if * i >= cls . len () { return None ; } match cls [* i] { None => None , Some (Class :: Int) => { * i += 1 ; Some (if size . bytes () < 8 { Reg { kind : RegKind :: Integer , size } } else { Reg :: i64 () }) } Some (Class :: Sse) => { let vec_len = 1 + cls [* i + 1 ..] . iter () . take_while (| & & c | c == Some (Class :: SseUp)) . count () ; * i += vec_len ; Some (if vec_len == 1 { match size . bytes () { 4 => Reg :: f32 () , _ => Reg :: f64 () , } } else { Reg { kind : RegKind :: Vector , size : Size :: from_bytes (8) * (vec_len as u64) } }) } Some (c) => unreachable ! ("reg_component: unhandled class {:?}" , c) , } }
};
}
