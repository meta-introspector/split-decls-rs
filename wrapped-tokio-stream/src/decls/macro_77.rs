macro_rules! deps {
    () => {
        StreamMap!();
    };
}

macro_rules! macro_77 {
    () => {
        deps!();
        pin_project ! { # [doc = " A `Stream` that wraps the values in an `Option`."] # [doc = ""] # [doc = " Whenever the wrapped stream yields an item, this stream yields that item"] # [doc = " wrapped in `Some`. When the inner stream ends, then this stream first"] # [doc = " yields a `None` item, and then this stream will also end."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Using `StreamNotifyClose` to handle closed streams with `StreamMap`."] # [doc = ""] # [doc = " ```"] # [doc = " use tokio_stream::{StreamExt, StreamMap, StreamNotifyClose};"] # [doc = ""] # [doc = " # #[tokio::main(flavor = \"current_thread\")]"] # [doc = " # async fn main() {"] # [doc = " let mut map = StreamMap::new();"] # [doc = " let stream = StreamNotifyClose::new(tokio_stream::iter(vec![0, 1]));"] # [doc = " let stream2 = StreamNotifyClose::new(tokio_stream::iter(vec![0, 1]));"] # [doc = " map.insert(0, stream);"] # [doc = " map.insert(1, stream2);"] # [doc = " while let Some((key, val)) = map.next().await {"] # [doc = "     match val {"] # [doc = "         Some(val) => println!(\"got {val:?} from stream {key:?}\"),"] # [doc = "         None => println!(\"stream {key:?} closed\"),"] # [doc = "     }"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] # [must_use = "streams do nothing unless polled"] pub struct StreamNotifyClose < S > { # [pin] inner : Option < S >, } }
    };
}

macro_77!()