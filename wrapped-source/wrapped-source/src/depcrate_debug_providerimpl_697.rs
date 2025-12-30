// Generated macro for impl_697 (impl)
macro_rules! Depcrate_debug_providerimpl_697 {
() => {
// Module: crate::debug_provider
// Provides: {"impl_697"}
// Dependencies: {}
impl < M : DataMarker > DataProvider < M > for DebugProvider where M :: DataStruct : Clone , { fn load (& self , _req : DataRequest) -> Result < DataResponse < M > , DataError > { use core :: any :: { type_name , Any , TypeId } ; use icu :: datetime :: provider :: neo :: * ; let type_id = TypeId :: of :: < M :: DataStruct > () ; let data : Box < dyn Any > = if type_id == TypeId :: of :: < YearNames > () { Box :: new (YearNames :: FixedEras (Default :: default ())) } else if type_id == TypeId :: of :: < MonthNames > () { Box :: new (MonthNames :: Linear (Default :: default ())) } else if type_id == TypeId :: of :: < LinearNames > () { Box :: new (LinearNames { names : Default :: default () , }) } else if type_id == TypeId :: of :: < [char ; 10] > () { Box :: new (['\0' ; 10]) } else if type_id == TypeId :: of :: < DecimalSymbols > () { Box :: new (DecimalSymbols :: new_en_for_testing ()) } else { panic ! ("Don't how how to create for debug: {}" , type_name ::< M :: DataStruct > ()) ; } ; let data : Box < M :: DataStruct > = data . downcast () . unwrap () ; let data : M :: DataStruct = (* data) . clone () ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (data) , }) } }
};
}
