use std::collections::BTreeMap;

use crate::db::scheme::ArticlesRow;

use super::scheme::OperationsRow;
use futures_util::TryStreamExt;
use tokio_postgres::{
    Client, Config, Error, NoTls, Statement,
    types::{ToSql, Type},
};
pub struct Inner {
    user: String,
    client: Client,

    select_from_operations: Statement,
    select_from_articles: Statement,
    select_from_balance: Statement,

    insert_to_operations: Statement,
    insert_to_articles: Statement,

    update_in_operations: Statement,
    update_in_articles: Statement,

    delete_from_operations: Statement,
    delete_from_articles: Statement,
    // create_balance: Statement,
    // remove_balance: Statement,
    // show_percents: Statement,
    // show_dynamics: Statement,
    // show_profit: Statement,
}
impl Inner {
    pub async fn new(user: String, password: String) -> Result<Self, Error> {
        let (client, connection) = Config::new()
            .host("/var/run/postgresql/")
            .dbname("budget")
            .user(&user)
            .password(password)
            .connect(NoTls)
            .await?;
        tokio::spawn(async move {
            if let Err(err) = connection.await {
                log::error!("Ошибка подключения к базе: {err}");
            }
        });
        let (
            select_from_operations,
            select_from_articles,
            select_from_balance,
            insert_to_operations,
            insert_to_articles,
            update_in_operations,
            update_in_articles,
            delete_from_operations,
            delete_from_articles,
            // create_balance,
            // remove_balance,
        ) = tokio::try_join!(
            Self::prepare_select_from_operations(&client),
            Self::prepare_select_from_articles(&client),
            Self::prepare_select_from_balance(&client),
            Self::prepare_insert_to_operations(&client),
            Self::prepare_insert_to_articles(&client),
            Self::prepare_update_in_operations(&client),
            Self::prepare_update_in_articles(&client),
            Self::prepare_delete_from_operations(&client),
            Self::prepare_delete_from_articles(&client),
            // Self::prepare_create_balance(&client),
            // Self::prepare_remove_balance(&client),
        )?;
        Ok(Self {
            user,
            client,
            select_from_operations,
            select_from_articles,
            select_from_balance,
            insert_to_operations,
            insert_to_articles,
            update_in_operations,
            update_in_articles,
            delete_from_operations,
            delete_from_articles,
            // create_balance,
            // remove_balance,
        })
    }
    pub fn user(&self) -> &str {
        &self.user
    }
    pub async fn select_from_operations(&self) -> Result<BTreeMap<i32, OperationsRow>, Error> {
        self.client
            .query_raw(&self.select_from_operations, NO_PARAMS)
            .await?
            .map_ok(|r| OperationsRow::new(r).unwrap())
            .try_collect()
            .await
    }
    pub async fn insert_to_operations(
        &self,
        row: OperationsRow,
    ) -> Result<BTreeMap<i32, OperationsRow>, Error> {
        self.client
            .execute(
                &self.insert_to_operations,
                &[&row.article_id, &row.debit, &row.credit, &row.create_date],
            )
            .await?;
        self.select_from_operations().await
    }
    pub async fn update_in_operations(
        &self,
        id: i32,
        row: OperationsRow,
    ) -> Result<BTreeMap<i32, OperationsRow>, Error> {
        self.client
            .execute(
                &self.update_in_operations,
                &[
                    &id,
                    &row.article_id,
                    &row.debit,
                    &row.credit,
                    &row.create_date,
                ],
            )
            .await?;
        self.select_from_operations().await
    }
    pub async fn delete_from_operations(
        &self,
        id: i32,
    ) -> Result<BTreeMap<i32, OperationsRow>, Error> {
        self.client
            .execute(&self.delete_from_operations, &[&id])
            .await?;
        self.select_from_operations().await
    }
    pub async fn select_from_articles(&self) -> Result<BTreeMap<i32, ArticlesRow>, Error> {
        self.client
            .query_raw(&self.select_from_articles, NO_PARAMS)
            .await?
            .map_ok(|r| ArticlesRow::new(r).unwrap())
            .try_collect()
            .await
    }
    pub async fn insert_to_articles(
        &self,
        row: ArticlesRow,
    ) -> Result<BTreeMap<i32, ArticlesRow>, Error> {
        self.client
            .execute(&self.insert_to_articles, &[&row.name])
            .await?;
        self.select_from_articles().await
    }
    pub async fn update_in_articles(
        &self,
        id: i32,
        row: ArticlesRow,
    ) -> Result<BTreeMap<i32, ArticlesRow>, Error> {
        self.client
            .execute(&self.update_in_articles, &[&id, &row.name])
            .await?;
        self.select_from_articles().await
    }
    pub async fn delete_from_articles(&self, id: i32) -> Result<BTreeMap<i32, ArticlesRow>, Error> {
        self.client
            .execute(&self.delete_from_articles, &[&id])
            .await?;
        self.select_from_articles().await
    }
    async fn prepare_select_from_operations(client: &Client) -> Result<Statement, Error> {
        client.prepare("SELECT * FROM public.operations").await
    }
    async fn prepare_select_from_articles(client: &Client) -> Result<Statement, Error> {
        client.prepare("SELECT * FROM public.articles").await
    }
    async fn prepare_select_from_balance(client: &Client) -> Result<Statement, Error> {
        client.prepare("SELECT * FROM public.balance").await
    }
    async fn prepare_insert_to_operations(client: &Client) -> Result<Statement, Error> {
        client
            .prepare_typed(
                "INSERT INTO public.operations( \
            	article_id, debit, credit, create_date)\
            	VALUES ($1, $2, $3, $4)",
                &[Type::INT4, Type::INT4, Type::INT4, Type::TIMESTAMP],
            )
            .await
    }
    async fn prepare_insert_to_articles(client: &Client) -> Result<Statement, Error> {
        client
            .prepare_typed(
                "INSERT INTO public.articles(name) VALUES ($1)",
                &[Type::VARCHAR],
            )
            .await
    }
    async fn prepare_update_in_operations(client: &Client) -> Result<Statement, Error> {
        client
            .prepare_typed(
                "UPDATE public.operations \
            	SET article_id=$2, debit=$3, credit=$4, create_date=$5 \
            	WHERE id=$1",
                &[
                    Type::INT4,
                    Type::INT4,
                    Type::INT4,
                    Type::INT4,
                    Type::TIMESTAMP,
                ],
            )
            .await
    }
    async fn prepare_update_in_articles(client: &Client) -> Result<Statement, Error> {
        client
            .prepare_typed(
                "UPDATE public.articles \
            	SET name=$2 \
            	WHERE id=$1",
                &[Type::INT4, Type::VARCHAR],
            )
            .await
    }

    async fn prepare_delete_from_operations(client: &Client) -> Result<Statement, Error> {
        client
            .prepare_typed("DELETE FROM public.operations WHERE id = $1", &[Type::INT4])
            .await
    }
    async fn prepare_delete_from_articles(client: &Client) -> Result<Statement, Error> {
        client
            .prepare_typed("DELETE FROM public.articles WHERE id = $1", &[Type::INT4])
            .await
    }
    // async fn prepare_create_balance(client: &Client) -> Result<Statement, Error> {
    //     todo!()
    // }
    // async fn prepare_remove_balance(client: &Client) -> Result<Statement, Error> {
    //     todo!()
    // }
}

// Говорим системе типов замолчать, когда взрослые разговаривают
const NO_PARAMS: Option<&(dyn ToSql + Sync)> = None;
