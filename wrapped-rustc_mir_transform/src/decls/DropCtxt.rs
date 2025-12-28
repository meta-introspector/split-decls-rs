macro_rules! deps {
    () => {
        DropElaborator!();
        Unwind!();
    };
}

macro_rules! DropCtxt {
    () => {
        deps!();
        # [derive (Debug)] struct DropCtxt < 'a , 'b , 'tcx , D > where D : DropElaborator < 'b , 'tcx > , { elaborator : & 'a mut D , source_info : SourceInfo , place : Place < 'tcx > , path : D :: Path , succ : BasicBlock , unwind : Unwind , dropline : Option < BasicBlock > , }
    };
}

DropCtxt!()