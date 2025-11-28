use crate::{
    app::drive_result_promise,
    db::{Db, scheme::ProfitPoint},
    promise_lite::PromiseLite,
};
use chrono::DateTime;
use egui::Color32;
use tokio_postgres::Error;
pub struct State {
    values: Option<Vec<ProfitPoint>>,
    error_message: Option<String>,
    result: Option<PromiseLite<Result<Vec<ProfitPoint>, Error>>>,
}
impl State {
    pub fn new(db: &Db) -> Self {
        Self {
            values: None,
            error_message: None,
            result: Some(db.show_profit()),
        }
    }
    pub fn view(&mut self, ui: &mut egui::Ui, db: &Db) {
        ui.heading("Прибыль");
        let enabled = self.result.is_none();
        if let Some(values) = &mut self.values {
            egui::containers::ScrollArea::new([true, true]).show(ui, |ui| {
                let size = ui.available_height();
                egui_plot::Plot::new("Profit")
                    .height(size / 2.0)
                    .clamp_grid(true)
                    .x_axis_label("Время")
                    .x_axis_formatter(|_, _| "".into())
                    .y_axis_label("Деньги")
                    .label_formatter(|_, point| {
                        format!(
                            "Время:{}\nДеньги:{}",
                            DateTime::from_timestamp(point.x as i64, 0)
                                .map(|d| d.to_string())
                                .unwrap_or_default(),
                            point.y
                        )
                    })
                    .show(ui, |plot_ui| {
                        let color = Color32::RED;
                        let points: Vec<_> = values.iter().map(|p| p.0).collect();
                        let line =
                            egui_plot::Line::new("Прибыль от времени", points.clone()).color(color);
                        let points = egui_plot::Points::new("Прибыль от времени", points)
                            .color(color)
                            .radius(5.0);
                        plot_ui.line(line);
                        plot_ui.points(points);
                    })
            });
        }
        let reload = egui::Button::new("Перезагрузить!");
        if ui.add_enabled(enabled, reload).clicked() {
            self.result = Some(db.show_profit())
        }
        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, error);
        }
    }
    pub fn drive(&mut self) {
        drive_result_promise!(
            self.result,
            Ok(values) => {
                self.values = Some(values);
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
