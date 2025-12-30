// Generated macro for FSEEncoder (struct)
macro_rules! Depcrate_fse_fse_encoderFSEEncoder {
() => {
// Module: crate::fse::fse_encoder
// Provides: {"FSEEncoder"}
// Dependencies: {}
pub (crate) struct FSEEncoder < 'output , V : AsMut < Vec < u8 > > > { pub (super) table : FSETable , writer : & 'output mut BitWriter < V > , }
};
}
