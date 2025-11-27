use tokio_postgres::{Error, Row};

#[derive(Clone, PartialEq, Default)]
pub struct OperationsRow {
    pub article_id: Option<i32>,
    pub balance_id: Option<i32>,
    pub debit: Option<i32>,
    pub credit: Option<i32>,
    pub create_date: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, PartialEq, Default)]
pub struct ArticlesRow {
    pub name: Option<String>,
}

#[derive(Clone, PartialEq, Default)]
pub struct BalanceRow {
    pub debit: Option<i32>,
    pub credit: Option<i32>,
    pub amount: Option<i32>,
    pub create_date: Option<chrono::NaiveDateTime>,
}

impl OperationsRow {
    pub fn new(row: Row) -> Result<(i32, Self), Error> {
        Ok((
            row.try_get("id")?,
            Self {
                article_id: row.try_get("article_id")?,
                balance_id: row.try_get("balance_id")?,
                debit: row.try_get("debit")?,
                credit: row.try_get("credit")?,
                create_date: row.try_get("create_date")?,
            },
        ))
    }
}
impl ArticlesRow {
    pub fn new(row: Row) -> Result<(i32, Self), Error> {
        Ok((
            row.try_get("id")?,
            Self {
                name: row.try_get("name")?,
            },
        ))
    }
}

impl BalanceRow {
    pub fn new(row: Row) -> Result<(i32, Self), Error> {
        Ok((
            row.try_get("id")?,
            Self {
                debit: row.try_get("debit")?,
                credit: row.try_get("credit")?,
                amount: row.try_get("amount")?,
                create_date: row.try_get("create_date")?,
            },
        ))
    }
}
