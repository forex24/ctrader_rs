use governor::{
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct RateLimitUnboundedReceiver<T> {
    pub rx: mpsc::UnboundedReceiver<T>,
    lim: RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
}

impl<T> RateLimitUnboundedReceiver<T> {
    pub fn new(rx: mpsc::UnboundedReceiver<T>, quota: Quota) -> Self {
        let lim = RateLimiter::direct(quota);

        Self { rx, lim }
    }

    pub async fn recv(&mut self) -> Option<T> {
        self.lim.until_ready().await;
        self.rx.recv().await
    }
}
