// Generated macro for file_stream (function)
macro_rules! Depcrate_filters_fsfile_stream {
() => {
// Module: crate::filters::fs
// Provides: {"file_stream"}
// Dependencies: {}
fn file_stream (mut file : TkFile , buf_size : usize , (start , end) : (u64 , u64) ,) -> impl Stream < Item = Result < Bytes , io :: Error > > + Send { use std :: io :: SeekFrom ; let seek = async move { if start != 0 { file . seek (SeekFrom :: Start (start)) . await ? ; } Ok (file) } ; seek . into_stream () . map (move | result | { let mut buf = BytesMut :: new () ; let mut len = end - start ; let mut f = match result { Ok (f) => f , Err (f) => return Either :: Left (stream :: once (future :: err (f))) , } ; Either :: Right (stream :: poll_fn (move | cx | { if len == 0 { return Poll :: Ready (None) ; } reserve_at_least (& mut buf , buf_size) ; let n = match ready ! (poll_read_buf (Pin :: new (& mut f) , cx , & mut buf)) { Ok (n) => n as u64 , Err (err) => { tracing :: debug ! ("file read error: {}" , err) ; return Poll :: Ready (Some (Err (err))) ; } } ; if n == 0 { tracing :: debug ! ("file read found EOF before expected length") ; return Poll :: Ready (None) ; } let mut chunk = buf . split () . freeze () ; if n > len { chunk = chunk . split_to (len as usize) ; len = 0 ; } else { len -= n ; } Poll :: Ready (Some (Ok (chunk))) })) }) . flatten () }
};
}
