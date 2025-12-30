// Generated macro for impl_130 (impl)
macro_rules! Depcrate_serverimpl_130 {
() => {
// Module: crate::server
// Provides: {"impl_130"}
// Dependencies: {}
impl < IO > Future for LazyConfigAcceptor < IO > where IO : AsyncRead + AsyncWrite + Unpin , { type Output = Result < StartHandshake < IO > , io :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . get_mut () ; loop { let io = match this . io . as_mut () { Some (io) => io , None => { return Poll :: Ready (Err (io :: Error :: new (io :: ErrorKind :: Other , "acceptor cannot be polled after acceptance" ,))) } } ; if let Some ((err , mut alert)) = this . alert . take () { match alert . write (& mut SyncWriteAdapter { io , cx }) { Err (e) if e . kind () == io :: ErrorKind :: WouldBlock => { this . alert = Some ((err , alert)) ; return Poll :: Pending ; } Ok (0) | Err (_) => { return Poll :: Ready (Err (io :: Error :: new (io :: ErrorKind :: InvalidData , err))) } Ok (_) => { this . alert = Some ((err , alert)) ; continue ; } } ; } let mut reader = SyncReadAdapter { io , cx } ; match this . acceptor . read_tls (& mut reader) { Ok (0) => return Err (io :: ErrorKind :: UnexpectedEof . into ()) . into () , Ok (_) => { } Err (e) if e . kind () == io :: ErrorKind :: WouldBlock => return Poll :: Pending , Err (e) => return Err (e) . into () , } match this . acceptor . accept () { Ok (Some (accepted)) => { let io = this . io . take () . unwrap () ; return Poll :: Ready (Ok (StartHandshake { accepted , io })) ; } Ok (None) => { } Err ((err , alert)) => { this . alert = Some ((err , alert)) ; } } } } }
};
}
