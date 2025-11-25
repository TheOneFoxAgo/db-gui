use tokio_postgres::Error;

use crate::{
    app::drive_result_promise,
    db::{
        Db,
        scheme::{OperationsRow, TableRow},
    },
    promise_lite::PromiseLite,
};
pub struct State {
    table: Option<Vec<OperationsRow>>,
    highlighted: Option<usize>,
    edited: Option<(usize, [String; OperationsRow::COLUMNS])>,
    error_message: Option<String>,
    results: Promises,
}
pub enum Response {
    ShowArticle(i32),
    ShowBalance(i32),
    None,
}
struct Promises {
    select: Option<PromiseLite<Result<Vec<OperationsRow>, Error>>>,
    insert: Option<PromiseLite<Result<u64, Error>>>,
    delete: Option<PromiseLite<Result<u64, Error>>>,
}
impl Default for State {
    fn default() -> Self {
        Self {
            table: None,
            highlighted: None,
            edited: None,
            error_message: None,
            results: Promises {
                select: None,
                insert: None,
                delete: None,
            },
        }
    }
}
impl State {
    pub fn view(&mut self, ui: &mut egui::Ui, db: &Db) -> Response {
        ui.heading("Операции");
        if let Some(table) = &self.table {
            egui::Grid::new("Operations").num_columns(6).show(ui, |ui| {
                for header in [
                    "id",
                    "article_id",
                    "balance_id",
                    "debit",
                    "credit",
                    "create_date",
                ] {
                    ui.strong(header);
                }
                ui.end_row();
                for row in table {
                    for label in row.to_string_array() {
                        ui.label(label);
                    }
                    ui.end_row();
                }
            });
        }
        if self.results.select.is_some() {
            ui.add_enabled(false, egui::Button::new("Загружаем..."));
        } else {
            if ui.button("Перезагрузить!").clicked() {
                self.results.select = Some(db.select_from_operations())
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
            self.results.select,
            Ok(table) => {
                self.table = Some(table);
            },
            Err(err) => self.set_err(err),
        );
    }
}
