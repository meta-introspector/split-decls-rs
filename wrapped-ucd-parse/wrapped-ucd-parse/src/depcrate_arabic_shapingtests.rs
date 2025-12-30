// Generated macro for tests (module)
macro_rules! Depcrate_arabic_shapingtests {
() => {
// Module: crate::arabic_shaping
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: common :: Codepoint ; use super :: { ArabicShaping , JoiningType } ; fn codepoint (n : u32) -> Codepoint { Codepoint :: from_u32 (n) . unwrap () } fn s (string : & str) -> String { string . to_string () } # [test] fn parse1 () { let line = "0600; ARABIC NUMBER SIGN; U; No_Joining_Group\n" ; let data : ArabicShaping = line . parse () . unwrap () ; assert_eq ! (data , ArabicShaping { codepoint : codepoint (0x0600) , schematic_name : s ("ARABIC NUMBER SIGN") , joining_type : JoiningType :: NonJoining , joining_group : s ("No_Joining_Group") }) ; } # [test] fn parse2 () { let line = "063D; FARSI YEH WITH INVERTED V ABOVE; D; FARSI YEH\n" ; let data : ArabicShaping = line . parse () . unwrap () ; assert_eq ! (data , ArabicShaping { codepoint : codepoint (0x063D) , schematic_name : s ("FARSI YEH WITH INVERTED V ABOVE") , joining_type : JoiningType :: DualJoining , joining_group : s ("FARSI YEH") }) ; } # [test] fn parse3 () { let line = "10D23; HANIFI ROHINGYA DOTLESS KINNA YA WITH DOT ABOVE; D; HANIFI ROHINGYA KINNA YA\n" ; let data : ArabicShaping = line . parse () . unwrap () ; assert_eq ! (data , ArabicShaping { codepoint : codepoint (0x10D23) , schematic_name : s ("HANIFI ROHINGYA DOTLESS KINNA YA WITH DOT ABOVE") , joining_type : JoiningType :: DualJoining , joining_group : s ("HANIFI ROHINGYA KINNA YA") }) ; } }
};
}
