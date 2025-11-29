use chrono::NaiveDateTime;
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

#[derive(Clone, PartialEq)]
pub struct PercentsBar {
    pub article_name: String,
    pub debit: f64,
    pub credit: f64,
}

#[derive(Clone, PartialEq)]
pub struct ProfitPoint(pub egui_plot::PlotPoint);

#[derive(Clone, PartialEq)]
pub struct DynamicsPoint {
    pub debit: egui_plot::PlotPoint,
    pub credit: egui_plot::PlotPoint,
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

impl PercentsBar {
    pub fn new(row: Row) -> Result<Self, Error> {
        let article_name: String = row.try_get("article_name")?;
        let debit: Option<f64> = row.try_get("debit")?;
        let credit: Option<f64> = row.try_get("credit")?;
        Ok(Self {
            article_name,
            debit: debit.unwrap_or_default(),
            credit: credit.unwrap_or_default(),
        })
    }
}
impl ProfitPoint {
    pub fn new(row: Row) -> Result<Self, Error> {
        let date: NaiveDateTime = row.try_get("create_date")?;
        let date = date.and_utc().timestamp() as f64;
        let profit: f64 = row.try_get("profit")?;
        Ok(Self(egui_plot::PlotPoint { x: date, y: profit }))
    }
}

impl DynamicsPoint {
    pub fn new(row: Row) -> Result<Self, Error> {
        let date: NaiveDateTime = row.try_get("create_date")?;
        let date = date.and_utc().timestamp() as f64;
        let debit: i64 = row.try_get("debit")?;
        let credit: i64 = row.try_get("credit")?;
        Ok(Self {
            debit: (egui_plot::PlotPoint::new(date, debit as f64)),
            credit: (egui_plot::PlotPoint::new(date, credit as f64)),
        })
    }
}
