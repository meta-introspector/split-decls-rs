macro_rules! deps {
    () => {
        VarKind!();
        LocalInfo!();
        LiveNodeKind!();
        IrMaps!();
        CaptureInfo!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl < 'tcx > IrMaps < 'tcx > { fn new (tcx : TyCtxt < 'tcx >) -> IrMaps < 'tcx > { IrMaps { tcx , live_node_map : HirIdMap :: default () , variable_map : HirIdMap :: default () , capture_info_map : Default :: default () , var_kinds : IndexVec :: new () , lnks : IndexVec :: new () , } } fn add_live_node (& mut self , lnk : LiveNodeKind) -> LiveNode { let ln = self . lnks . push (lnk) ; debug ! ("{:?} is of kind {}" , ln , live_node_kind_to_string (lnk , self . tcx)) ; ln } fn add_live_node_for_node (& mut self , hir_id : HirId , lnk : LiveNodeKind) { let ln = self . add_live_node (lnk) ; self . live_node_map . insert (hir_id , ln) ; debug ! ("{:?} is node {:?}" , ln , hir_id) ; } fn add_variable (& mut self , vk : VarKind) -> Variable { let v = self . var_kinds . push (vk) ; match vk { Local (LocalInfo { id : node_id , .. }) | Param (node_id , _) | Upvar (node_id , _) => { self . variable_map . insert (node_id , v) ; } } debug ! ("{:?} is {:?}" , v , vk) ; v } fn variable (& self , hir_id : HirId , span : Span) -> Variable { match self . variable_map . get (& hir_id) { Some (& var) => var , None => { span_bug ! (span , "no variable registered for id {:?}" , hir_id) ; } } } fn variable_name (& self , var : Variable) -> Symbol { match self . var_kinds [var] { Local (LocalInfo { name , .. }) | Param (_ , name) | Upvar (_ , name) => name , } } fn variable_is_shorthand (& self , var : Variable) -> bool { match self . var_kinds [var] { Local (LocalInfo { is_shorthand , .. }) => is_shorthand , Param (..) | Upvar (..) => false , } } fn set_captures (& mut self , hir_id : HirId , cs : Vec < CaptureInfo >) { self . capture_info_map . insert (hir_id , Rc :: new (cs)) ; } fn collect_shorthand_field_ids (& self , pat : & hir :: Pat < 'tcx >) -> HirIdSet { let mut shorthand_field_ids = HirIdSet :: default () ; pat . walk_always (| pat | { if let hir :: PatKind :: Struct (_ , fields , _) = pat . kind { let short = fields . iter () . filter (| f | f . is_shorthand) ; shorthand_field_ids . extend (short . map (| f | f . pat . hir_id)) ; } }) ; shorthand_field_ids } fn add_from_pat (& mut self , pat : & hir :: Pat < 'tcx >) { let shorthand_field_ids = self . collect_shorthand_field_ids (pat) ; pat . each_binding (| _ , hir_id , _ , ident | { self . add_live_node_for_node (hir_id , VarDefNode (ident . span , hir_id)) ; self . add_variable (Local (LocalInfo { id : hir_id , name : ident . name , is_shorthand : shorthand_field_ids . contains (& hir_id) , })) ; }) ; } }
    };
}

impl_295!();