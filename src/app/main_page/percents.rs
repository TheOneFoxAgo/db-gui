use crate::{
    app::drive_result_promise,
    db::{Db, scheme::PercentsBar},
    promise_lite::PromiseLite,
};
use egui_plot::BarChart;
use tokio_postgres::Error;
pub struct State {
    values: Option<Bars>,
    error_message: Option<String>,
    result: Option<PromiseLite<Result<Vec<PercentsBar>, Error>>>,
}
pub struct Bars {
    debits: Vec<egui_plot::Bar>,
    credits: Vec<egui_plot::Bar>,
}
impl State {
    pub fn new(db: &Db) -> Self {
        Self {
            values: None,
            error_message: None,
            result: Some(db.show_percents()),
        }
    }
    pub fn view(&mut self, ui: &mut egui::Ui, db: &Db) {
        ui.heading("Проценты");
        let enabled = self.result.is_none();
        if let (Some(values)) = (&mut self.values) {
            egui::containers::ScrollArea::new([true, true]).show(ui, |ui| {
                let size = ui.available_height() / 2.0;
                ui.horizontal(|ui| {
                    egui_plot::Plot::new("Debit Percents")
                        .height(size)
                        .width(size)
                        .clamp_grid(true)
                        .x_axis_label("Статьи")
                        .y_axis_label("Проценты доходов")
                        .y_axis_formatter(|m, _| format!("{}%", m.value))
                        .show(ui, |plot_ui| {
                            plot_ui.bar_chart(BarChart::new(
                                "Проценты по прибылям",
                                values.debits.clone(),
                            ));
                        });
                    egui_plot::Plot::new("Credit Percents")
                        .height(size)
                        .width(size)
                        .clamp_grid(true)
                        .x_axis_label("Статьи")
                        .y_axis_label("Проценты расходов")
                        .y_axis_formatter(|m, _| format!("{}%", m.value))
                        .show(ui, |plot_ui| {
                            plot_ui.bar_chart(BarChart::new(
                                "Проценты по расходам",
                                values.credits.clone(),
                            ));
                        });
                });
            });
        }
        let reload = egui::Button::new("Перезагрузить!");
        if ui.add_enabled(enabled, reload).clicked() {
            self.result = Some(db.show_percents())
        }
        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, error);
        }
    }
    pub fn drive(&mut self) {
        drive_result_promise!(
            self.result,
            Ok(values) => {
                let (debits, credits) = values
                        .into_iter()
                        .enumerate()
                        .map(|(i, b)| (
                            egui_plot::Bar::new(i as f64, b.debit).name(b.article_name.clone()),
                            egui_plot::Bar::new(i as f64, b.credit).name(b.article_name),
                        ))
                        .collect();
                self.values = Some(Bars { debits, credits });
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
