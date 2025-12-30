// Generated macro for impl_214 (impl)
macro_rules! Depcrate_paximpl_214 {
() => {
// Module: crate::pax
// Provides: {"impl_214"}
// Dependencies: {}
impl < 'entry > Iterator for PaxExtensions < 'entry > { type Item = io :: Result < PaxExtension < 'entry > > ; fn next (& mut self) -> Option < io :: Result < PaxExtension < 'entry > > > { let line = match self . data . next () { Some ([]) => return None , Some (line) => line , None => return None , } ; Some (line . iter () . position (| b | * b == b' ') . and_then (| i | { str :: from_utf8 (& line [.. i]) . ok () . and_then (| len | len . parse :: < usize > () . ok () . map (| j | (i + 1 , j))) }) . and_then (| (kvstart , reported_len) | { if line . len () + 1 == reported_len { line [kvstart ..] . iter () . position (| b | * b == b'=') . map (| equals | (kvstart , equals)) } else { None } }) . map (| (kvstart , equals) | PaxExtension { key : & line [kvstart .. kvstart + equals] , value : & line [kvstart + equals + 1 ..] , }) . ok_or_else (| | other ("malformed pax extension")) ,) } }
};
}
