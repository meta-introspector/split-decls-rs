use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(no_salsa_async_drops))]
impl<T> Drop for Parse<T> {
    fn drop(&mut self) {
        let Some(green) = self.green.take() else {
            return;
        };
        static PARSE_DROP_THREAD: std::sync::OnceLock<
            std::sync::mpsc::Sender<GreenNode>,
        > = std::sync::OnceLock::new();
        PARSE_DROP_THREAD
            .get_or_init(|| {
                let (sender, receiver) = std::sync::mpsc::channel::<GreenNode>();
                std::thread::Builder::new()
                    .name("ParseNodeDropper".to_owned())
                    .spawn(move || receiver.iter().for_each(drop))
                    .unwrap();
                sender
            })
            .send(green)
            .unwrap();
    }
}
