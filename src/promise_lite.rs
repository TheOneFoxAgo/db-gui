use tokio::task::{JoinError, JoinHandle};

pub struct PromiseLite<T>(JoinHandle<T>);
impl<T> PromiseLite<T>
where
    T: Send + 'static,
{
    pub fn spawn(future: impl Future<Output = T> + Send + 'static) -> Self {
        Self(tokio::spawn(future))
    }
    pub fn is_finished(&self) -> bool {
        self.0.is_finished()
    }
    pub fn block_take(self) -> Result<T, JoinError> {
        tokio::runtime::Handle::current().block_on(self.0)
    }
}
