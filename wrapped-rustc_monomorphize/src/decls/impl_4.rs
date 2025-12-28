macro_rules! deps {
    () => {
        UsageMap!();
        MonoItems!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'tcx > UsageMap < 'tcx > { fn new () -> UsageMap < 'tcx > { UsageMap { used_map : Default :: default () , user_map : Default :: default () } } fn record_used < 'a > (& mut self , user_item : MonoItem < 'tcx > , used_items : & 'a MonoItems < 'tcx >) where 'tcx : 'a , { for used_item in used_items . items () { self . user_map . entry (used_item) . or_default () . push (user_item) ; } assert ! (self . used_map . insert (user_item , used_items . items () . collect ()) . is_none ()) ; } pub (crate) fn get_user_items (& self , item : MonoItem < 'tcx >) -> & [MonoItem < 'tcx >] { self . user_map . get (& item) . map (| items | items . as_slice ()) . unwrap_or (& []) } # [doc = " Internally iterate over all inlined items used by `item`."] pub (crate) fn for_each_inlined_used_item < F > (& self , tcx : TyCtxt < 'tcx > , item : MonoItem < 'tcx > , mut f : F ,) where F : FnMut (MonoItem < 'tcx >) , { let used_items = self . used_map . get (& item) . unwrap () ; for used_item in used_items . iter () { let is_inlined = used_item . instantiation_mode (tcx) == InstantiationMode :: LocalCopy ; if is_inlined { f (* used_item) ; } } } }
    };
}

impl_4!();