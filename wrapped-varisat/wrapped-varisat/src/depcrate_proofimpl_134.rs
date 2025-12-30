// Generated macro for impl_134 (impl)
macro_rules! Depcrate_proofimpl_134 {
() => {
// Module: crate::proof
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'a > Proof < 'a > { # [doc = " Start writing proof steps to the given target with the given format."] pub fn write_proof (& mut self , target : impl Write + 'a , format : ProofFormat) { self . format = Some (format) ; self . target = BufWriter :: new (Box :: new (target)) } # [doc = " Begin checking proof steps."] pub fn begin_checking (& mut self) { if self . checker . is_none () { self . checker = Some (Checker :: new ()) } } # [doc = " Add a [`ProofProcessor`]."] # [doc = ""] # [doc = " See also [`Checker::add_processor`]."] pub fn add_processor (& mut self , processor : & 'a mut dyn ProofProcessor) { self . begin_checking () ; self . checker . as_mut () . unwrap () . add_processor (processor) ; } # [doc = " Whether proof generation is active."] pub fn is_active (& self) -> bool { self . checker . is_some () || self . format . is_some () } # [doc = " Are we emitting or checking our native format."] pub fn native_format (& self) -> bool { self . checker . is_some () || matches ! (self . format , Some (ProofFormat :: Varisat)) } # [doc = " Whether clause hashes are required for steps that support them."] pub fn clause_hashes_required (& self) -> bool { self . native_format () } # [doc = " Whether found models are included in the proof."] pub fn models_in_proof (& self) -> bool { self . native_format () } }
};
}
