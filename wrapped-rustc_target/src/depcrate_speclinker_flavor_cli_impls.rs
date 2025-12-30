// Generated macro for linker_flavor_cli_impls (macro)
macro_rules! Depcrate_speclinker_flavor_cli_impls {
() => {
// Module: crate::spec
// Provides: {"linker_flavor_cli_impls"}
// Dependencies: {}
macro_rules ! linker_flavor_cli_impls { ($ (($ ($ flavor : tt) *) $ string : literal) *) => (impl LinkerFlavorCli { const fn all () -> &'static [LinkerFlavorCli] { & [$ ($ ($ flavor) *,) *] } pub const fn one_of () -> &'static str { concat ! ("one of: " , $ ($ string , " " ,) *) } pub fn desc (self) -> &'static str { match self { $ ($ ($ flavor) * => $ string ,) * } } } impl FromStr for LinkerFlavorCli { type Err = String ; fn from_str (s : & str) -> Result < LinkerFlavorCli , Self :: Err > { Ok (match s { $ ($ string => $ ($ flavor) *,) * _ => return Err (format ! ("invalid linker flavor, allowed values: {}" , Self :: one_of ())) , }) } }) }
};
}
