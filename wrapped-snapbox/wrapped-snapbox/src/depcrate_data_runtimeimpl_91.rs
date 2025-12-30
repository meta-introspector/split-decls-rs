// Generated macro for impl_91 (impl)
macro_rules! Depcrate_data_runtimeimpl_91 {
() => {
// Module: crate::data::runtime
// Provides: {"impl_91"}
// Dependencies: {}
impl Patchwork { fn new (text : String) -> Patchwork { Patchwork { text , indels : BTreeMap :: new () , } } fn patch (& mut self , mut range : std :: ops :: Range < usize > , patch : & str) -> std :: io :: Result < () > { let key : OrdRange = range . clone () . into () ; match self . indels . entry (key) { std :: collections :: btree_map :: Entry :: Vacant (entry) => { entry . insert ((patch . len () , patch . to_owned ())) ; } std :: collections :: btree_map :: Entry :: Occupied (entry) => { if entry . get () . 1 == patch { return Ok (()) ; } else { return Err (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "cannot update as it was already modified" ,)) ; } } } let (delete , insert) = self . indels . iter () . take_while (| (delete , _) | delete . start < range . start) . map (| (delete , (insert , _)) | (delete . end - delete . start , insert)) . fold ((0usize , 0usize) , | (x1 , y1) , (x2 , y2) | (x1 + x2 , y1 + y2)) ; for pos in & mut [& mut range . start , & mut range . end] { * * pos -= delete ; * * pos += insert ; } self . text . replace_range (range , patch) ; Ok (()) } }
};
}
