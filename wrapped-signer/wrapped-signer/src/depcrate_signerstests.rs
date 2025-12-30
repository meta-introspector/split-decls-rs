// Generated macro for tests (module)
macro_rules! Depcrate_signerstests {
() => {
// Module: crate::signers
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; struct Foo ; impl Signer for Foo { fn try_pubkey (& self) -> Result < Pubkey , SignerError > { Ok (Pubkey :: default ()) } fn try_sign_message (& self , _message : & [u8]) -> Result < Signature , SignerError > { Ok (Signature :: default ()) } fn is_interactive (& self) -> bool { false } } struct Bar ; impl Signer for Bar { fn try_pubkey (& self) -> Result < Pubkey , SignerError > { Ok (Pubkey :: default ()) } fn try_sign_message (& self , _message : & [u8]) -> Result < Signature , SignerError > { Ok (Signature :: default ()) } fn is_interactive (& self) -> bool { false } } # [test] fn test_dyn_keypairs_compile () { let xs : Vec < Box < dyn Signer > > = vec ! [Box :: new (Foo { }) , Box :: new (Bar { })] ; assert_eq ! (xs . sign_message (b"") , vec ! [Signature :: default () , Signature :: default ()] ,) ; let xs_ref : & [Box < dyn Signer >] = & xs ; assert_eq ! (Signers :: sign_message (xs_ref , b"") , vec ! [Signature :: default () , Signature :: default ()] ,) ; } # [test] fn test_dyn_keypairs_by_ref_compile () { let foo = Foo { } ; let bar = Bar { } ; let xs : Vec < & dyn Signer > = vec ! [& foo , & bar] ; assert_eq ! (xs . sign_message (b"") , vec ! [Signature :: default () , Signature :: default ()] ,) ; let xs_ref : & [& dyn Signer] = & xs ; assert_eq ! (Signers :: sign_message (xs_ref , b"") , vec ! [Signature :: default () , Signature :: default ()] ,) ; } }
};
}
