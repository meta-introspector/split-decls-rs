macro_rules! deps {
    () => {
        ToSocketAddrs!();
    };
}

macro_rules! sealed {
    () => {
        deps!();
        pub (crate) mod sealed { # ! [doc = " The contents of this trait are intended to remain private and __not__"] # ! [doc = " part of the `ToSocketAddrs` public API. The details will change over"] # ! [doc = " time."] use std :: future :: Future ; use std :: io ; use std :: net :: SocketAddr ; # [doc (hidden)] pub trait ToSocketAddrsPriv { type Iter : Iterator < Item = SocketAddr > + Send + 'static ; type Future : Future < Output = io :: Result < Self :: Iter > > + Send + 'static ; fn to_socket_addrs (& self , internal : Internal) -> Self :: Future ; } # [allow (missing_debug_implementations)] pub struct Internal ; cfg_net ! { use crate :: blocking :: JoinHandle ; use std :: option ; use std :: pin :: Pin ; use std :: task :: { ready , Context , Poll } ; use std :: vec ; # [doc (hidden)] # [derive (Debug)] pub struct MaybeReady (pub (super) State) ; # [derive (Debug)] pub (super) enum State { Ready (Option < SocketAddr >) , Blocking (JoinHandle < io :: Result < vec :: IntoIter < SocketAddr >>>) , } # [doc (hidden)] # [derive (Debug)] pub enum OneOrMore { One (option :: IntoIter < SocketAddr >) , More (vec :: IntoIter < SocketAddr >) , } impl Future for MaybeReady { type Output = io :: Result < OneOrMore >; fn poll (mut self : Pin <& mut Self >, cx : & mut Context <'_ >) -> Poll < Self :: Output > { match self . 0 { State :: Ready (ref mut i) => { let iter = OneOrMore :: One (i . take () . into_iter ()) ; Poll :: Ready (Ok (iter)) } State :: Blocking (ref mut rx) => { let res = ready ! (Pin :: new (rx) . poll (cx)) ?. map (OneOrMore :: More) ; Poll :: Ready (res) } } } } impl Iterator for OneOrMore { type Item = SocketAddr ; fn next (& mut self) -> Option < Self :: Item > { match self { OneOrMore :: One (i) => i . next () , OneOrMore :: More (i) => i . next () , } } fn size_hint (& self) -> (usize , Option < usize >) { match self { OneOrMore :: One (i) => i . size_hint () , OneOrMore :: More (i) => i . size_hint () , } } } } }
    };
}

sealed!();