// Generated macro for ConnectionStage (trait)
macro_rules! Depcrate_quic_io_connection_stageConnectionStage {
() => {
// Module: crate::quic::io::connection_stage
// Provides: {"ConnectionStage"}
// Dependencies: {}
# [doc = " Represents the current lifecycle stage of a [quiche::Connection]."] # [doc = " Implementors of this trait inform the underlying I/O loop as to how to"] # [doc = " behave."] # [doc = ""] # [doc = " The I/O loop will always handle sending/receiving packets - this trait"] # [doc = " simply serves to augment its functionality. For example, an established"] # [doc = " HTTP/3 connection may want its `on_read` to include handing packets off to"] # [doc = " an [ApplicationOverQuic]."] # [doc = ""] # [doc = " To prevent borrow checker conflicts, we inject a `qconn` into all methods."] # [doc = " This also simplifies state transitions, since the `IoWorker` must maintain"] # [doc = " ownership over the connection in order to read, gather, and flush from it."] pub trait ConnectionStage : Send + Debug { fn on_read < A : ApplicationOverQuic > (& mut self , _received_packets : bool , _qconn : & mut QuicheConnection , _ctx : & mut ConnectionStageContext < A > ,) -> QuicResult < () > { Ok (()) } fn on_flush < A : ApplicationOverQuic > (& mut self , _qconn : & mut QuicheConnection , _ctx : & mut ConnectionStageContext < A > ,) -> ControlFlow < QuicResult < () > > { ControlFlow :: Continue (()) } fn wait_deadline (& mut self) -> Option < Instant > { None } fn post_wait (& self , _qconn : & mut QuicheConnection ,) -> ControlFlow < QuicResult < () > > { ControlFlow :: Continue (()) } }
};
}
