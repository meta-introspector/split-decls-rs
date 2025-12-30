// Generated macro for SccacheTransport (struct)
macro_rules! Depcrate_serverSccacheTransport {
() => {
// Module: crate::server
// Provides: {"SccacheTransport"}
// Dependencies: {}
# [doc = " Implementation of `Stream + Sink` that tokio-proto is expecting"] # [doc = ""] # [doc = " This type is composed of a few layers:"] # [doc = ""] # [doc = " * First there's `I`, the I/O object implementing `AsyncRead` and"] # [doc = "   `AsyncWrite`"] # [doc = " * Next that's framed using the `length_delimited` module in tokio-io giving"] # [doc = "   us a `Sink` and `Stream` of `BytesMut`."] # [doc = " * Next that sink/stream is wrapped in `ReadBincode` which will cause the"] # [doc = "   `Stream` implementation to switch from `BytesMut` to `Request` by parsing"] # [doc = "   the bytes  bincode."] # [doc = " * Finally that sink/stream is wrapped in `WriteBincode` which will cause the"] # [doc = "   `Sink` implementation to switch from `BytesMut` to `Response` meaning that"] # [doc = "   all `Response` types pushed in will be converted to `BytesMut` and pushed"] # [doc = "   below."] struct SccacheTransport < I : AsyncRead + AsyncWrite + Unpin > { inner : Framed < futures :: stream :: ErrInto < futures :: sink :: SinkErrInto < tokio_util :: codec :: Framed < I , LengthDelimitedCodec > , Bytes , Error , > , Error , > , Request , Response , BincodeCodec , > , }
};
}
