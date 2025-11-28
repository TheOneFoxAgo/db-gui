use std::collections::BTreeMap;

use tokio_postgres::Error;

use crate::{
    app::{drive_result_promise, main_page::option_to_string},
    db::{Db, scheme::BalanceRow},
    promise_lite::PromiseLite,
};
pub struct State {
    table: Option<BTreeMap<i32, BalanceRow>>,
    error_message: Option<String>,
    result: Option<PromiseLite<Result<BTreeMap<i32, BalanceRow>, Error>>>,
}
impl State {
    pub fn new(db: &Db) -> Self {
        Self {
            table: None,
            error_message: None,
            result: Some(db.select_from_balance()),
        }
    }
    pub fn view(&mut self, ui: &mut egui::Ui, db: &Db) {
        ui.heading("Статьи");
        let enabled = self.result.is_none();
        if let Some(table) = &mut self.table {
            let headers = ["id", "create_date", "debit", "credit", "amount"];
            egui::containers::ScrollArea::new([true, true]).show(ui, |ui| {
                egui::Grid::new("Balance")
                    .num_columns(headers.len())
                    .show(ui, |ui| {
                        for header in headers.iter() {
                            ui.strong(*header);
                        }
                        ui.end_row();
                        for (id, row) in table {
                            ui.label(id.to_string());
                            ui.label(option_to_string(row.create_date.as_ref()));
                            ui.label(option_to_string(row.debit.as_ref()));
                            ui.label(option_to_string(row.credit.as_ref()));
                            ui.label(option_to_string(row.amount.as_ref()));
                            ui.end_row();
                        }
                    });
            });
        }
        ui.horizontal(|ui| {
            let create = egui::Button::new("Сформировать!");
            if ui.add_enabled(enabled, create).clicked() {
                self.result = Some(db.create_balance());
            }
            let remove = egui::Button::new("Расформировать!");
            if ui.add_enabled(enabled, remove).clicked() {
                self.result = Some(db.remove_balance())
            }
        });
        let reload = egui::Button::new("Перезагрузить!");
        if ui.add_enabled(enabled, reload).clicked() {
            self.result = Some(db.select_from_balance())
        }
        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, error);
        }
    }
    pub fn drive(&mut self) {
        drive_result_promise!(
            self.result,
            Ok(values) => {
                self.table = Some(values);
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
