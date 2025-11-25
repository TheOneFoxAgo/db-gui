mod inner;
pub mod scheme;

use std::sync::Arc;

use crate::promise_lite::PromiseLite;
use tokio_postgres::Error;

macro_rules! wrap {
    ($self:ident, |$clone:ident| $future:expr) => {{
        let $clone = $self.clone();
        PromiseLite::spawn(async move {
            let res = $future.await;
            $clone.ctx.request_repaint();
            res
        })
    }};
}
#[derive(Clone)]
pub struct Db {
    inner: Arc<inner::Inner>,
    ctx: egui::Context,
}
impl Db {
    pub fn new(
        user: String,
        password: String,
        ctx: egui::Context,
    ) -> PromiseLite<Result<Self, Error>> {
        PromiseLite::spawn(async move {
            inner::Inner::new(user, password).await.map(|i| Db {
                inner: Arc::new(i),
                ctx,
            })
        })
    }
    pub fn user(&self) -> &str {
        self.inner.user()
    }
    pub fn select_from_operations(&self) -> PromiseLite<Result<Vec<scheme::OperationsRow>, Error>> {
        wrap!(self, |clone| clone.inner.select_from_operations())
    }
}
