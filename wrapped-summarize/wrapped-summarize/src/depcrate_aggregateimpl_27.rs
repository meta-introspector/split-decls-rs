// Generated macro for impl_27 (impl)
macro_rules! Depcrate_aggregateimpl_27 {
() => {
// Module: crate::aggregate
// Provides: {"impl_27"}
// Dependencies: {}
impl SamplePoint < WithParent < Event < '_ > > > { fn timestamp (& self) -> SystemTime { let timestamp = match self . event () . this . payload { EventPayload :: Timestamp (t) => t , _ => unreachable ! () , } ; match (self , timestamp) { (SamplePoint :: Start (_) , Timestamp :: Interval { start , .. }) => start , (SamplePoint :: End (_) , Timestamp :: Interval { end , .. }) => end , (SamplePoint :: Instant (_) , Timestamp :: Instant (time)) => time , _ => panic ! ("SamplePoint::timestamp: event timestamp doesn't match \
                 `SamplePoint` variant, in `SamplePoint::{:?}`" , self) , } } }
};
}
