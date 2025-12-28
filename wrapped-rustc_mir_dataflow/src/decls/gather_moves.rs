macro_rules! deps {
    () => {
        MoveData!();
        MoveDataBuilder!();
    };
}

macro_rules! gather_moves {
    () => {
        deps!();
        pub (super) fn gather_moves < 'tcx > (body : & Body < 'tcx > , tcx : TyCtxt < 'tcx > , filter : impl Fn (Ty < 'tcx >) -> bool ,) -> MoveData < 'tcx > { let mut builder = MoveDataBuilder :: new (body , tcx , filter) ; builder . gather_args () ; for (bb , block) in body . basic_blocks . iter_enumerated () { for (i , stmt) in block . statements . iter () . enumerate () { builder . loc = Location { block : bb , statement_index : i } ; builder . gather_statement (stmt) ; } builder . loc = Location { block : bb , statement_index : block . statements . len () } ; builder . gather_terminator (block . terminator ()) ; } builder . finalize () }
    };
}

gather_moves!()