use tokio_postgres::{Row, types::ToSql};

pub trait TableRow<const C: usize> {
    const COLUMNS: usize = C;
    fn to_string_array(&self) -> [String; C];
}

pub struct OperationsRow {
    pub id: i32,
    pub article_id: Option<i32>,
    pub balance_id: Option<i32>,
    pub debit: Option<i32>,
    pub credit: Option<i32>,
    pub create_date: Option<chrono::NaiveDateTime>,
}

pub struct ArticlesRow {
    pub id: i32,
    pub name: Option<String>,
}

pub struct BalanceRow {
    pub id: i32,
    pub debit: Option<i32>,
    pub credit: Option<i32>,
    pub amount: Option<i32>,
    pub create_date: Option<chrono::NaiveDateTime>,
}

impl TableRow<6> for OperationsRow {
    fn to_string_array(&self) -> [String; 6] {
        [
            self.id.to_string(),
            empty_or_string(&self.article_id),
            empty_or_string(&self.balance_id),
            empty_or_string(&self.debit),
            empty_or_string(&self.credit),
            empty_or_string(&self.create_date),
        ]
    }
}
impl TryFrom<Row> for OperationsRow {
    type Error = anyhow::Error;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get("id")?,
            article_id: value.try_get("article_id")?,
            balance_id: value.try_get("balance_id")?,
            debit: value.try_get("debit")?,
            credit: value.try_get("credit")?,
            create_date: value.try_get("create_date")?,
        })
    }
}

impl TableRow<2> for ArticlesRow {
    fn to_string_array(&self) -> [String; 2] {
        [self.id.to_string(), empty_or_string(&self.name)]
    }
}
impl TryFrom<Row> for ArticlesRow {
    type Error = anyhow::Error;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get("id")?,
            name: value.try_get("name")?,
        })
    }
}

impl TableRow<5> for BalanceRow {
    fn to_string_array(&self) -> [String; 5] {
        [
            self.id.to_string(),
            empty_or_string(&self.debit),
            empty_or_string(&self.credit),
            empty_or_string(&self.amount),
            empty_or_string(&self.create_date),
        ]
    }
}
impl TryFrom<Row> for BalanceRow {
    type Error = anyhow::Error;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get("id")?,
            debit: value.try_get("debit")?,
            credit: value.try_get("credit")?,
            amount: value.try_get("amount")?,
            create_date: value.try_get("create_date")?,
        })
    }
}

fn empty_or_string<T: ToString>(option: &Option<T>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => "".into(),
    }
}
