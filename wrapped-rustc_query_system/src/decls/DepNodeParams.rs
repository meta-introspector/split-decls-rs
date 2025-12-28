macro_rules! deps {
    () => {
        FingerprintStyle!();
        DepNode!();
        DepContext!();
    };
}

macro_rules! DepNodeParams {
    () => {
        deps!();
        pub trait DepNodeParams < Tcx : DepContext > : fmt :: Debug + Sized { fn fingerprint_style () -> FingerprintStyle ; # [doc = " This method turns the parameters of a DepNodeConstructor into an opaque"] # [doc = " Fingerprint to be used in DepNode."] # [doc = " Not all DepNodeParams support being turned into a Fingerprint (they"] # [doc = " don't need to if the corresponding DepNode is anonymous)."] fn to_fingerprint (& self , _ : Tcx) -> Fingerprint { panic ! ("Not implemented. Accidentally called on anonymous node?") } fn to_debug_str (& self , tcx : Tcx) -> String ; # [doc = " This method tries to recover the query key from the given `DepNode`,"] # [doc = " something which is needed when forcing `DepNode`s during red-green"] # [doc = " evaluation. The query system will only call this method if"] # [doc = " `fingerprint_style()` is not `FingerprintStyle::Opaque`."] # [doc = " It is always valid to return `None` here, in which case incremental"] # [doc = " compilation will treat the query as having changed instead of forcing it."] fn recover (tcx : Tcx , dep_node : & DepNode) -> Option < Self > ; }
    };
}

DepNodeParams!();