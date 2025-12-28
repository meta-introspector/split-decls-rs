macro_rules! deps {
    () => {
        MoveData!();
    };
}

macro_rules! MoveDataBuilder {
    () => {
        deps!();
        struct MoveDataBuilder < 'a , 'tcx , F > { body : & 'a Body < 'tcx > , loc : Location , tcx : TyCtxt < 'tcx > , data : MoveData < 'tcx > , filter : F , }
    };
}

MoveDataBuilder!()