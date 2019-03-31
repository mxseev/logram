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

#[cfg(test)]
mod tests {
    use failure::err_msg;
    use futures::Stream;

    use super::result_channel;

    #[test]
    fn main() {
        let (tx, rx) = result_channel();
        let mut stream = rx.wait();

        tx.unbounded_send(Ok(true)).unwrap();
        assert!(stream.next().unwrap().is_ok());

        tx.unbounded_send(Err(err_msg("oh no"))).unwrap();
        assert!(stream.next().unwrap().is_err());
    }
}
