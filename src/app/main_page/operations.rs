mod table;
use std::collections::BTreeMap;

use tokio_postgres::Error;

use crate::{
    app::drive_result_promise,
    db::{
        Db,
        scheme::{ArticlesRow, OperationsRow},
    },
    promise_lite::PromiseLite,
};
pub struct State {
    table: Option<table::State>,
    error_message: Option<String>,
    result: Option<PromiseLite<Result<BTreeMap<i32, OperationsRow>, Error>>>,
}
pub enum Response {
    ShowArticle(i32),
    ShowBalance(i32),
    None,
}
impl Default for State {
    fn default() -> Self {
        Self {
            table: None,
            error_message: None,
            result: None,
        }
    }
}
impl State {
    pub fn view(
        &mut self,
        ui: &mut egui::Ui,
        db: &Db,
        articles: Option<&BTreeMap<i32, ArticlesRow>>,
    ) -> Response {
        ui.heading("Операции");
        let is_waiting = self.result.is_some();
        if let (Some(table), Some(articles)) = (&mut self.table, articles) {
            if let Some(response) = table.show(ui, !is_waiting, articles) {
                match response {
                    table::Response::Update(id, operations_row) => {
                        self.result = Some(db.update_in_operations(id, operations_row))
                    }
                    table::Response::Delete(id) => {
                        self.result = Some(db.delete_from_operations(id))
                    }
                    table::Response::Insert(operations_row) => {
                        self.result = Some(db.insert_to_operations(operations_row))
                    }
                }
            }
        }
        if self.result.is_some() {
            ui.add_enabled(false, egui::Button::new("Загружаем..."));
        } else {
            if ui.button("Перезагрузить!").clicked() {
                self.result = Some(db.select_from_operations())
            }
        }
        self.process_results();
        Response::None
    }
    fn set_err(&mut self, err: impl ToString) {
        let message = err.to_string();
        log::error!("{}", message);
        self.error_message = Some(message);
    }
    fn process_results(&mut self) {
        drive_result_promise!(
            self.result,
            Ok(values) => {
                self.table = Some(table::State::new(values));
            },
            Err(err) => self.set_err(err),
        );
    }
}
