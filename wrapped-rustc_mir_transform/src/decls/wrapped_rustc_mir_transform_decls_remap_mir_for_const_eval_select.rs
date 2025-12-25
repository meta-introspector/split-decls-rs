// !!! Formatting failed for this module. The following code is unformatted. !!!
// !!! Error: prettyplease::unparse panicked for file output2/wrapped-rustc_mir_transform/src/decls/wrapped_rustc_mir_transform_decls_remap_mir_for_const_eval_select.rs: not implemented: Pat::Verbatim `box ConstOperand { ref const_ , .. }` !!!

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn remap_mir_for_const_eval_select<'tcx>(
    tcx: TyCtxt<'tcx>,
    mut body: Body<'tcx>,
    context: hir::Constness,
) -> Body<'tcx> {
    for bb in body.basic_blocks.as_mut().iter_mut() {
        let terminator = bb.terminator.as_mut().expect("invalid terminator");
        match terminator.kind {
            TerminatorKind::Call {
                func: Operand::Constant(box ConstOperand { ref const_, .. }),
                ref mut args,
                destination,
                target,
                unwind,
                fn_span,
                ..
            } if let ty::FnDef(def_id, _) = *const_.ty().kind()
                && tcx.is_intrinsic(def_id, sym::const_eval_select) =>
            {
                let Ok([tupled_args, called_in_const, called_at_rt]) = take_array(args) else {
                    unreachable!()
                };
                let ty = tupled_args.node.ty(&body.local_decls, tcx);
                let fields = ty.tuple_fields();
                let num_args = fields.len();
                let func = if context == hir::Constness::Const {
                    called_in_const
                } else {
                    called_at_rt
                };
                let (method, place): (fn(Place<'tcx>) -> Operand<'tcx>, Place<'tcx>) =
                    match tupled_args.node {
                        Operand::Constant(_) => {
                            let local = body.local_decls.push(LocalDecl::new(ty, fn_span));
                            bb.statements.push(Statement::new(
                                SourceInfo::outermost(fn_span),
                                StatementKind::Assign(Box::new((
                                    local.into(),
                                    Rvalue::Use(tupled_args.node.clone()),
                                ))),
                            ));
                            (Operand::Move, local.into())
                        }
                        Operand::Move(place) => (Operand::Move, place),
                        Operand::Copy(place) => (Operand::Copy, place),
                    };
                let place_elems = place.projection;
                let arguments = (0..num_args)
                    .map(|x| {
                        let mut place_elems = place_elems.to_vec();
                        place_elems.push(ProjectionElem::Field(x.into(), fields[x]));
                        let projection = tcx.mk_place_elems(&place_elems);
                        let place = Place {
                            local: place.local,
                            projection,
                        };
                        Spanned {
                            node: method(place),
                            span: DUMMY_SP,
                        }
                    })
                    .collect();
                terminator.kind = TerminatorKind::Call {
                    func: func.node,
                    args: arguments,
                    destination,
                    target,
                    unwind,
                    call_source: CallSource::Misc,
                    fn_span,
                };
            }
            _ => {}
        }
    }
    body
}
