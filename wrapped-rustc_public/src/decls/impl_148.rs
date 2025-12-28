macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: AssertMessage < 'tcx > { type T = crate :: mir :: AssertMessage ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: mir :: AssertKind ; match self { AssertKind :: BoundsCheck { len , index } => crate :: mir :: AssertMessage :: BoundsCheck { len : len . stable (tables , cx) , index : index . stable (tables , cx) , } , AssertKind :: Overflow (bin_op , op1 , op2) => crate :: mir :: AssertMessage :: Overflow (bin_op . stable (tables , cx) , op1 . stable (tables , cx) , op2 . stable (tables , cx) ,) , AssertKind :: OverflowNeg (op) => { crate :: mir :: AssertMessage :: OverflowNeg (op . stable (tables , cx)) } AssertKind :: DivisionByZero (op) => { crate :: mir :: AssertMessage :: DivisionByZero (op . stable (tables , cx)) } AssertKind :: RemainderByZero (op) => { crate :: mir :: AssertMessage :: RemainderByZero (op . stable (tables , cx)) } AssertKind :: ResumedAfterReturn (coroutine) => { crate :: mir :: AssertMessage :: ResumedAfterReturn (coroutine . stable (tables , cx)) } AssertKind :: ResumedAfterPanic (coroutine) => { crate :: mir :: AssertMessage :: ResumedAfterPanic (coroutine . stable (tables , cx)) } AssertKind :: ResumedAfterDrop (coroutine) => { crate :: mir :: AssertMessage :: ResumedAfterDrop (coroutine . stable (tables , cx)) } AssertKind :: MisalignedPointerDereference { required , found } => { crate :: mir :: AssertMessage :: MisalignedPointerDereference { required : required . stable (tables , cx) , found : found . stable (tables , cx) , } } AssertKind :: NullPointerDereference => crate :: mir :: AssertMessage :: NullPointerDereference , AssertKind :: InvalidEnumConstruction (source) => { crate :: mir :: AssertMessage :: InvalidEnumConstruction (source . stable (tables , cx)) } } } }
    };
}

impl_148!()