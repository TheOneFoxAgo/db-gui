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
    pub debit: egui_plot::Bar,
    pub credit: egui_plot::Bar,
}

#[derive(Clone, PartialEq)]
pub struct ProfitPoint(pub [f64; 2]);

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
        let position: i32 = row.try_get("article_id")?;
        let debit: Option<f64> = row.try_get("debit")?;
        let credit: Option<f64> = row.try_get("credit")?;
        Ok(Self {
            debit: egui_plot::Bar::new(position.into(), debit.unwrap_or(0.0)),
            credit: egui_plot::Bar::new(position.into(), credit.unwrap_or(0.0)),
        })
    }
}
impl ProfitPoint {
    pub fn new(row: Row) -> Result<Self, Error> {
        let date: NaiveDateTime = row.try_get("create_date")?;
        let date = date.and_utc().timestamp() as f64;
        let profit: f64 = row.try_get("profit")?;
        Ok(Self([date, profit]))
    }
}

impl DynamicsPoint {
    pub fn new(row: Row) -> Result<Self, Error> {
        let date: NaiveDateTime = row.try_get("create_date")?;
        let date = date.and_utc().timestamp() as f64;
        let debit: i32 = row.try_get("debit")?;
        let credit: i32 = row.try_get("credit")?;
        Ok(Self {
            debit: (egui_plot::PlotPoint::new(date, debit)),
            credit: (egui_plot::PlotPoint::new(date, credit)),
        })
    }
}
