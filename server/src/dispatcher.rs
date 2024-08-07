//! Stores all the implementations for the server side of .proto files.
//! The gRPC service calls are mainly only going to be responsible for inserting
//! itself in the work queue sorted in best order
//!
//! Each modules are responsible for implementing single .proto files.

pub mod helloworld;

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex, OnceLock},
};

pub use helloworld::Hello;
use priority_queue::PriorityQueue;
use tower::{Layer, Service};

pub static mut EVENT_QUEUE: OnceLock<EventQueue> = OnceLock::new();

/// Service that runs after the standard gRPC calls.
/// After executing the gRPC dispatchers in the inner layer, run the highest
/// priority event stored in queue
#[derive(Clone)]
pub struct Event<S>
where
    S: Clone,
{
    inner: S,
}

#[derive(Clone)]
pub struct EventLayer;

#[derive(PartialEq, Eq, Hash)]
pub struct EventPayload {
    event: EventDescription,
    data: Vec<u8>,
}

pub struct EventQueue {
    tree: Arc<Mutex<PriorityQueue<EventPayload, u64>>>,
}

#[derive(PartialEq, Eq, Hash)]
pub enum EventDescription {
    Script,
    Engine,
    Stream,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Attempted to dispatch from empty event queue")]
    Empty,
}

impl<S, Req, Res> Service<Req> for Event<S>
where
    S: Service<Req, Response = Res> + Clone + Send + 'static,
    S::Future: Send + 'static,
    Req: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let response = inner.call(req).await?;
            EventQueue::dispatch_event();

            Ok(response)
        })
    }
}

impl<S> Layer<S> for EventLayer
where
    S: Clone,
{
    type Service = Event<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Event { inner }
    }
}

impl EventPayload {
    fn dispatch(self) -> Result<(), Error> {
        Ok(())
    }
}

impl EventQueue {
    fn new() -> Self {
        Self {
            tree: Arc::new(Mutex::new(PriorityQueue::new())),
        }
    }

    fn dispatch_event() -> Result<(), Error> {
        let queue = unsafe { EVENT_QUEUE.get_or_init(EventQueue::new) };

        let (event, _) = {
            let mut guard = queue
                .tree
                .lock()
                .expect("Panic means something went terribly wrong in other threads");
            guard.pop().ok_or_else(|| Error::Empty)?
        };

        event.dispatch()?;

        Ok(())
    }
}
