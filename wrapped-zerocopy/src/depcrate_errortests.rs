// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_send_sync () { # [allow (dead_code)] fn is_send_sync < T : Send + Sync > (_t : T) { } # [allow (dead_code)] fn alignment_err_is_send_sync < Src : Send + Sync , Dst > (err : AlignmentError < Src , Dst >) { is_send_sync (err) } # [allow (dead_code)] fn size_err_is_send_sync < Src : Send + Sync , Dst > (err : SizeError < Src , Dst >) { is_send_sync (err) } # [allow (dead_code)] fn validity_err_is_send_sync < Src : Send + Sync , Dst : TryFromBytes > (err : ValidityError < Src , Dst > ,) { is_send_sync (err) } # [allow (dead_code)] fn convert_error_is_send_sync < Src : Send + Sync , Dst : TryFromBytes > (err : ConvertError < AlignmentError < Src , Dst > , SizeError < Src , Dst > , ValidityError < Src , Dst > , > ,) { is_send_sync (err) } } # [test] fn test_eq_partial_eq_clone () { # [allow (dead_code)] fn is_eq_partial_eq_clone < T : Eq + PartialEq + Clone > (_t : T) { } # [allow (dead_code)] fn alignment_err_is_eq_partial_eq_clone < Src : Eq + PartialEq + Clone , Dst > (err : AlignmentError < Src , Dst > ,) { is_eq_partial_eq_clone (err) } # [allow (dead_code)] fn size_err_is_eq_partial_eq_clone < Src : Eq + PartialEq + Clone , Dst > (err : SizeError < Src , Dst > ,) { is_eq_partial_eq_clone (err) } # [allow (dead_code)] fn validity_err_is_eq_partial_eq_clone < Src : Eq + PartialEq + Clone , Dst : TryFromBytes > (err : ValidityError < Src , Dst > ,) { is_eq_partial_eq_clone (err) } # [allow (dead_code)] fn convert_error_is_eq_partial_eq_clone < Src : Eq + PartialEq + Clone , Dst : TryFromBytes > (err : ConvertError < AlignmentError < Src , Dst > , SizeError < Src , Dst > , ValidityError < Src , Dst > , > ,) { is_eq_partial_eq_clone (err) } } # [test] fn alignment_display () { # [repr (C , align (128))] struct Aligned { bytes : [u8 ; 128] , } impl_known_layout ! (elain :: Align ::< 8 >) ; let aligned = Aligned { bytes : [0 ; 128] } ; let bytes = & aligned . bytes [1 ..] ; let addr = crate :: util :: AsAddress :: addr (bytes) ; assert_eq ! (AlignmentError ::< _ , elain :: Align ::< 8 >>:: new_checked (bytes) . to_string () , format ! ("The conversion failed because the address of the source is not a multiple of the alignment of the destination type.\n\
            \nSource type: &[u8]\
            \nSource address: 0x{:x} (a multiple of 1)\
            \nDestination type: elain::Align<8>\
            \nDestination alignment: 8" , addr)) ; let bytes = & aligned . bytes [2 ..] ; let addr = crate :: util :: AsAddress :: addr (bytes) ; assert_eq ! (AlignmentError ::< _ , elain :: Align ::< 8 >>:: new_checked (bytes) . to_string () , format ! ("The conversion failed because the address of the source is not a multiple of the alignment of the destination type.\n\
            \nSource type: &[u8]\
            \nSource address: 0x{:x} (a multiple of 2)\
            \nDestination type: elain::Align<8>\
            \nDestination alignment: 8" , addr)) ; let bytes = & aligned . bytes [3 ..] ; let addr = crate :: util :: AsAddress :: addr (bytes) ; assert_eq ! (AlignmentError ::< _ , elain :: Align ::< 8 >>:: new_checked (bytes) . to_string () , format ! ("The conversion failed because the address of the source is not a multiple of the alignment of the destination type.\n\
            \nSource type: &[u8]\
            \nSource address: 0x{:x} (a multiple of 1)\
            \nDestination type: elain::Align<8>\
            \nDestination alignment: 8" , addr)) ; let bytes = & aligned . bytes [4 ..] ; let addr = crate :: util :: AsAddress :: addr (bytes) ; assert_eq ! (AlignmentError ::< _ , elain :: Align ::< 8 >>:: new_checked (bytes) . to_string () , format ! ("The conversion failed because the address of the source is not a multiple of the alignment of the destination type.\n\
            \nSource type: &[u8]\
            \nSource address: 0x{:x} (a multiple of 4)\
            \nDestination type: elain::Align<8>\
            \nDestination alignment: 8" , addr)) ; } # [test] fn size_display () { assert_eq ! (SizeError ::< _ , [u8] >:: new (& [0u8 ; 2] [..]) . to_string () , "The conversion failed because the source was incorrectly sized to complete the conversion into the destination type.\n\
            \nSource type: &[u8]\
            \nSource size: 2 bytes\
            \nDestination type: [u8]") ; assert_eq ! (SizeError ::< _ , [u8 ; 2] >:: new (& [0u8 ; 1] [..]) . to_string () , "The conversion failed because the source was incorrectly sized to complete the conversion into the destination type.\n\
            \nSource type: &[u8]\
            \nSource size: 1 byte\
            \nDestination size: 2 bytes\
            \nDestination type: [u8; 2]") ; } # [test] fn validity_display () { assert_eq ! (ValidityError ::< _ , bool >:: new (& [2u8 ; 1] [..]) . to_string () , "The conversion failed because the source bytes are not a valid value of the destination type.\n\
            \n\
            Destination type: bool") ; } }
};
}
