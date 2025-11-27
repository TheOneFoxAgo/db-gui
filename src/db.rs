mod inner;
pub mod scheme;

use std::{collections::BTreeMap, sync::Arc};

use crate::{
    db::scheme::{ArticlesRow, OperationsRow},
    promise_lite::PromiseLite,
};
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
    pub fn select_from_operations(
        &self,
    ) -> PromiseLite<Result<BTreeMap<i32, OperationsRow>, Error>> {
        wrap!(self, |clone| clone.inner.select_from_operations())
    }
    pub fn update_in_operations(
        &self,
        id: i32,
        row: OperationsRow,
    ) -> PromiseLite<Result<BTreeMap<i32, OperationsRow>, Error>> {
        wrap!(self, |clone| clone.inner.update_in_operations(id, row))
    }
    pub fn insert_to_operations(
        &self,
        row: OperationsRow,
    ) -> PromiseLite<Result<BTreeMap<i32, OperationsRow>, Error>> {
        wrap!(self, |clone| clone.inner.insert_to_operations(row))
    }
    pub fn delete_from_operations(
        &self,
        id: i32,
    ) -> PromiseLite<Result<BTreeMap<i32, OperationsRow>, Error>> {
        wrap!(self, |clone| clone.inner.delete_from_operations(id))
    }
    pub fn select_from_articles(&self) -> PromiseLite<Result<BTreeMap<i32, ArticlesRow>, Error>> {
        wrap!(self, |clone| clone.inner.select_from_articles())
    }
    pub fn update_in_articles(
        &self,
        id: i32,
        row: ArticlesRow,
    ) -> PromiseLite<Result<BTreeMap<i32, ArticlesRow>, Error>> {
        wrap!(self, |clone| clone.inner.update_in_articles(id, row))
    }
    pub fn insert_to_articles(
        &self,
        row: ArticlesRow,
    ) -> PromiseLite<Result<BTreeMap<i32, ArticlesRow>, Error>> {
        wrap!(self, |clone| clone.inner.insert_to_articles(row))
    }
    pub fn delete_from_articles(
        &self,
        id: i32,
    ) -> PromiseLite<Result<BTreeMap<i32, ArticlesRow>, Error>> {
        wrap!(self, |clone| clone.inner.delete_from_articles(id))
    }
}
