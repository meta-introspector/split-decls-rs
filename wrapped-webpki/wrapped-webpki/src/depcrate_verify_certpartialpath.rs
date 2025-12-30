// Generated macro for PartialPath (struct)
macro_rules! Depcrate_verify_certPartialPath {
() => {
// Module: crate::verify_cert
// Provides: {"PartialPath"}
// Dependencies: {}
# [doc = " A path for consideration in path building."] # [doc = ""] # [doc = " This represents a partial path because it does not yet contain the trust anchor. It stores"] # [doc = " the end-entity certificates, and an array of intermediate certificates."] pub (crate) struct PartialPath < 'a > { end_entity : & 'a EndEntityCert < 'a > , # [doc = " Intermediate certificates, in order from end-entity to trust anchor."] # [doc = ""] # [doc = " Invariant: all values below `used` are `Some`."] intermediates : [Option < Cert < 'a > > ; MAX_SUB_CA_COUNT] , # [doc = " The number of `Some` values in `intermediates`."] # [doc = ""] # [doc = " The next `Cert` passed to `push()` will be placed at `intermediates[used]`."] # [doc = " If this value is 0, the path contains only the end-entity certificate."] used : usize , }
};
}
