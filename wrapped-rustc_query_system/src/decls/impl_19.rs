macro_rules! deps {
    () => {
        DepNodeParams!();
        DepContext!();
        DepKind!();
        DepNode!();
        FingerprintStyle!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl DepNode { # [doc = " Creates a new, parameterless DepNode. This method will assert"] # [doc = " that the DepNode corresponding to the given DepKind actually"] # [doc = " does not require any parameters."] pub fn new_no_params < Tcx > (tcx : Tcx , kind : DepKind) -> DepNode where Tcx : super :: DepContext , { debug_assert_eq ! (tcx . fingerprint_style (kind) , FingerprintStyle :: Unit) ; DepNode { kind , hash : Fingerprint :: ZERO . into () } } pub fn construct < Tcx , Key > (tcx : Tcx , kind : DepKind , arg : & Key) -> DepNode where Tcx : super :: DepContext , Key : DepNodeParams < Tcx > , { let hash = arg . to_fingerprint (tcx) ; let dep_node = DepNode { kind , hash : hash . into () } ; # [cfg (debug_assertions)] { if ! tcx . fingerprint_style (kind) . reconstructible () && (tcx . sess () . opts . unstable_opts . incremental_info || tcx . sess () . opts . unstable_opts . query_dep_graph) { tcx . dep_graph () . register_dep_node_debug_str (dep_node , | | arg . to_debug_str (tcx)) ; } } dep_node } # [doc = " Construct a DepNode from the given DepKind and DefPathHash. This"] # [doc = " method will assert that the given DepKind actually requires a"] # [doc = " single DefId/DefPathHash parameter."] pub fn from_def_path_hash < Tcx > (tcx : Tcx , def_path_hash : DefPathHash , kind : DepKind) -> Self where Tcx : super :: DepContext , { debug_assert ! (tcx . fingerprint_style (kind) == FingerprintStyle :: DefPathHash) ; DepNode { kind , hash : def_path_hash . 0 . into () } } }
    };
}

impl_19!();