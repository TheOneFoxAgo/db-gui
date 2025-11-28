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
impl State {
    pub fn new(db: &Db) -> Self {
        Self {
            table: None,
            error_message: None,
            result: Some(db.select_from_operations()),
        }
    }
    pub fn view(
        &mut self,
        ui: &mut egui::Ui,
        db: &Db,
        articles: Option<&BTreeMap<i32, ArticlesRow>>,
    ) {
        ui.heading("Операции");
        let enabled = self.result.is_none();
        if let (Some(table), Some(articles)) = (&mut self.table, articles) {
            if let Some(response) = table.show(ui, enabled, articles) {
                match response {
                    table::Response::Update(id, operations_row) => {
                        self.result = Some(db.update_in_operations(id, operations_row))
                    }
                    table::Response::Delete(id) => {
                        log::info!("Удаляем ряд с id: {}", id);
                        self.result = Some(db.delete_from_operations(id))
                    }
                    table::Response::Insert(operations_row) => {
                        self.result = Some(db.insert_to_operations(operations_row))
                    }
                }
            }
        }
        ui.horizontal(|ui| {
            let insert = egui::Button::new("Добавить!");
            if ui
                .add_enabled(
                    enabled && self.table.as_ref().is_some_and(|t| !t.is_changing()),
                    insert,
                )
                .clicked()
            {
                if let Some(t) = &mut self.table {
                    t.insert_new_row();
                }
            }
            let reload = egui::Button::new("Перезагрузить!");
            if ui.add_enabled(enabled, reload).clicked() {
                self.result = Some(db.select_from_operations())
            }
        });
        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, error);
        }
    }
    pub fn drive(&mut self) {
        drive_result_promise!(
            self.result,
            Ok(values) => {
                self.table = Some(table::State::new(values));
                self.error_message = None;
            },
            Err(err) => self.set_err(err),
        );
    }
    fn set_err(&mut self, err: impl std::error::Error) {
        let message = format!("{err:?}");
        log::error!("{}", message);
        self.error_message = Some(message);
    }
}
