// Generated macro for sealed (module)
macro_rules! Depcrate_filters_replysealed {
() => {
// Module: crate::filters::reply
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { use super :: { WithDefaultHeader , WithHeader , WithHeaders } ; use crate :: generic :: { Func , One } ; use crate :: reply :: { Reply , Reply_ } ; # [derive (Clone)] # [allow (missing_debug_implementations)] pub struct WithHeader_ { pub (super) with : WithHeader , } impl < R : Reply > Func < One < R > > for WithHeader_ { type Output = Reply_ ; fn call (& self , args : One < R >) -> Self :: Output { let mut resp = args . 0 . into_response () ; resp . headers_mut () . insert (& self . with . name , self . with . value . clone ()) ; Reply_ (resp) } } # [derive (Clone)] # [allow (missing_debug_implementations)] pub struct WithHeaders_ { pub (super) with : WithHeaders , } impl < R : Reply > Func < One < R > > for WithHeaders_ { type Output = Reply_ ; fn call (& self , args : One < R >) -> Self :: Output { let mut resp = args . 0 . into_response () ; for (name , value) in & * self . with . headers { resp . headers_mut () . insert (name , value . clone ()) ; } Reply_ (resp) } } # [derive (Clone)] # [allow (missing_debug_implementations)] pub struct WithDefaultHeader_ { pub (super) with : WithDefaultHeader , } impl < R : Reply > Func < One < R > > for WithDefaultHeader_ { type Output = Reply_ ; fn call (& self , args : One < R >) -> Self :: Output { let mut resp = args . 0 . into_response () ; resp . headers_mut () . entry (& self . with . name) . or_insert_with (| | self . with . value . clone ()) ; Reply_ (resp) } } }
};
}
