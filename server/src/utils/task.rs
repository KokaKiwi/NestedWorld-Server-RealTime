/*
use mioco::JoinHandle;
use futures::{Async, Future, Poll};
use std::sync::mpsc as sync;

pub struct Task<T> {
    handle: sync::Receiver<T>,
}

impl<T> Future for Task<T> {
    type Item = T;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        unimplemented!()
    }
}
*/
