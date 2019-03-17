use failure::{err_msg, Error};
use futures::{
    sync::mpsc::{self, UnboundedSender},
    Stream,
};

type Sender<T> = UnboundedSender<Result<T, Error>>;

pub fn result_channel<T>() -> (Sender<T>, impl Stream<Item = T, Error = Error>) {
    let (tx, rx) = mpsc::unbounded();
    let new_rx = rx
        .map_err(|_| err_msg("result_channel error"))
        .and_then(|result| result);

    (tx, new_rx)
}
