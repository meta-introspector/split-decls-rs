macro_rules! SerdeStructVisitor {
    () => {
        # [doc = " Implements `tracing_core::field::Visit` for some `serde::ser::SerializeStruct`."] # [derive (Debug)] pub struct SerdeStructVisitor < S : SerializeStruct > { serializer : S , state : Result < () , S :: Error > , }
    };
}

SerdeStructVisitor!();