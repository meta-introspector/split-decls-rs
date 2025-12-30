// Generated macro for find_instrs (function)
macro_rules! Depcratefind_instrs {
() => {
// Module: crate
// Provides: {"find_instrs"}
// Dependencies: {}
fn find_instrs (attrs : & [syn :: Attribute]) -> Vec < String > { struct AssertInstr { instr : Option < String > , } impl syn :: parse :: Parse for AssertInstr { fn parse (input : syn :: parse :: ParseStream < '_ >) -> syn :: Result < Self > { let _ = input . parse :: < syn :: Meta > () . unwrap () ; let _ = input . parse :: < Token ! [,] > () . unwrap () ; match input . parse :: < syn :: Ident > () { Ok (ident) if ident == "assert_instr" => { } _ => { while ! input . is_empty () { drop (input . parse :: < proc_macro2 :: TokenStream > ()) ; } return Ok (Self { instr : None }) ; } } let instrs ; parenthesized ! (instrs in input) ; let mut instr = String :: new () ; while ! instrs . is_empty () { if let Ok (lit) = instrs . parse :: < syn :: LitStr > () { instr . push_str (& lit . value ()) ; } else if let Ok (ident) = instrs . call (syn :: Ident :: parse_any) { instr . push_str (& ident . to_string ()) ; } else if instrs . parse :: < Token ! [.] > () . is_ok () { instr . push ('.') ; } else if instrs . parse :: < Token ! [,] > () . is_ok () { drop (instrs . parse :: < proc_macro2 :: TokenStream > ()) ; break ; } else { return Err (input . error ("failed to parse instruction")) ; } } Ok (Self { instr : Some (instr) }) } } attrs . iter () . filter_map (| a | { if let syn :: Meta :: List (ref l) = a . meta { if l . path . is_ident ("cfg_attr") { Some (l) } else { None } } else { None } }) . filter_map (| l | syn :: parse2 :: < AssertInstr > (l . tokens . clone ()) . unwrap () . instr) . collect () }
};
}
