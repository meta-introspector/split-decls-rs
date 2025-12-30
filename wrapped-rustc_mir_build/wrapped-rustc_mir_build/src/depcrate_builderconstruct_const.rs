// Generated macro for construct_const (function)
macro_rules! Depcrate_builderconstruct_const {
() => {
// Module: crate::builder
// Provides: {"construct_const"}
// Dependencies: {}
fn construct_const < 'a , 'tcx > (tcx : TyCtxt < 'tcx > , def : LocalDefId , thir : & 'a Thir < 'tcx > , expr : ExprId , const_ty : Ty < 'tcx > ,) -> Body < 'tcx > { let hir_id = tcx . local_def_id_to_hir_id (def) ; let (span , const_ty_span) = match tcx . hir_node (hir_id) { Node :: Item (hir :: Item { kind : hir :: ItemKind :: Static (_ , _ , ty , _) | hir :: ItemKind :: Const (_ , _ , ty , _) , span , .. }) | Node :: ImplItem (hir :: ImplItem { kind : hir :: ImplItemKind :: Const (ty , _) , span , .. }) | Node :: TraitItem (hir :: TraitItem { kind : hir :: TraitItemKind :: Const (ty , Some (_)) , span , .. }) => (* span , ty . span) , Node :: AnonConst (ct) => (ct . span , ct . span) , Node :: ConstBlock (_) => { let span = tcx . def_span (def) ; (span , span) } Node :: Item (hir :: Item { kind : hir :: ItemKind :: GlobalAsm { .. } , span , .. }) => (* span , * span) , _ => span_bug ! (tcx . def_span (def) , "can't build MIR for {:?}" , def) , } ; let infcx = tcx . infer_ctxt () . build (TypingMode :: non_body_analysis ()) ; let mut builder = Builder :: new (thir , infcx , def , hir_id , span , 0 , const_ty , const_ty_span , None) ; let mut block = START_BLOCK ; block = builder . expr_into_dest (Place :: return_place () , block , expr) . into_block () ; let source_info = builder . source_info (span) ; builder . cfg . terminate (block , source_info , TerminatorKind :: Return) ; builder . build_drop_trees () ; builder . finish () }
};
}
