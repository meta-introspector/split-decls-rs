// Generated macro for compress (function)
macro_rules! Depcrate_compresscompress {
() => {
// Module: crate::compress
// Provides: {"compress"}
// Dependencies: {}
# [inline] pub (crate) fn compress (state : & mut [u32 ; 5] , ctx : & mut DetectionState , blocks : & [[u8 ; BLOCK_SIZE]] ,) { let mut block_u32 = [0u32 ; BLOCK_SIZE / 4] ; for block in blocks . iter () { ctx . ihv1 . copy_from_slice (& * state) ; for (o , chunk) in block_u32 . iter_mut () . zip (block . chunks_exact (4)) { * o = u32 :: from_be_bytes (chunk . try_into () . unwrap ()) ; } let DetectionState { m1 , state_58 , state_65 , .. } = ctx ; compression_states (state , & block_u32 , m1 , state_58 , state_65) ; let ubc_mask = if ctx . ubc_check { crate :: ubc_check :: ubc_check (& ctx . m1) } else { 0xFFFFFFFF } ; if ubc_mask != 0 { let mut ihvtmp = [0u32 ; 5] ; for dv_type in & crate :: ubc_check :: SHA1_DVS { if ubc_mask & (1 << dv_type . maskb) != 0 { for ((m2 , m1) , dm) in ctx . m2 . iter_mut () . zip (ctx . m1 . iter ()) . zip (dv_type . dm . iter ()) { * m2 = m1 ^ dm ; } let DetectionState { ihv2 , m2 , state_58 , state_65 , .. } = ctx ; recompression_step (dv_type . testt , ihv2 , & mut ihvtmp , m2 , match dv_type . testt { Testt :: T58 => state_58 , Testt :: T65 => state_65 , } ,) ; if (0 == xor (& ihvtmp , & * state)) || (ctx . reduced_round_collision && 0 == xor (& ctx . ihv1 , & ctx . ihv2)) { ctx . found_collision = true ; if ctx . safe_hash { compression_w (state , & ctx . m1) ; compression_w (state , & ctx . m1) ; } break ; } } } } } }
};
}
