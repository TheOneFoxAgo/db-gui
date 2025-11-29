use std::collections::{BTreeMap, HashSet};

use crate::{
    app::{drive_result_promise, main_page::option_to_string},
    db::{
        Db,
        scheme::{ArticlesRow, DynamicsPoint},
    },
    promise_lite::PromiseLite,
};
use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use egui::Color32;
use egui_plot::PlotPoints;
use tokio_postgres::Error;
pub struct State {
    start: NaiveDate,
    end: NaiveDate,
    chosen_articles: HashSet<i32>,
    values: Option<Points>,
    error_message: Option<String>,
    result: Option<PromiseLite<Result<Vec<DynamicsPoint>, Error>>>,
}
#[derive(Clone)]
struct Points {
    debits: Vec<egui_plot::PlotPoint>,
    credits: Vec<egui_plot::PlotPoint>,
}
impl State {
    pub fn new() -> Self {
        let now = Local::now()
            .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .unwrap()
            .date_naive();
        Self {
            start: now,
            end: now,
            chosen_articles: HashSet::new(),
            values: None,
            error_message: None,
            result: None,
        }
    }
    pub fn view(
        &mut self,
        ui: &mut egui::Ui,
        db: &Db,
        articles: Option<&BTreeMap<i32, ArticlesRow>>,
    ) {
        ui.heading("Прибыль");
        let enabled = self.result.is_none();
        if let Some(articles) = articles {
            egui::containers::ScrollArea::new([true, true]).show(ui, |ui| {
                let size = ui.available_height();
                ui.horizontal(|ui| {
                    Self::table(ui, articles, &mut self.chosen_articles);
                    if let Some(values) = &self.values {
                        Self::plot(ui, values.clone(), size / 2.0);
                    }
                })
            });
        }
        ui.horizontal(|ui| {
            ui.add(egui_extras::DatePickerButton::new(&mut self.start).id_salt("start"));
            ui.add(egui_extras::DatePickerButton::new(&mut self.end).id_salt("end"));
        });
        let reload = egui::Button::new("Перезагрузить!");
        if ui.add_enabled(enabled, reload).clicked() {
            self.result = Some(db.show_dynamics(
                self.chosen_articles.iter().copied().collect(),
                self.start,
                self.end,
            ))
        }
        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, error);
        }
    }
    pub fn drive(&mut self) {
        drive_result_promise!(
            self.result,
            Ok(values) => {
                self.values = Some(
                    Points {
                        debits: values.iter().map(|v| v.debit).collect(),
                        credits: values.iter().map(|v| v.credit).collect()
                    }
                );
                self.error_message = None;
            },
            Err(err) => self.set_err(err),
        );
    }
    fn table(ui: &mut egui::Ui, articles: &BTreeMap<i32, ArticlesRow>, chosen: &mut HashSet<i32>) {
        let header = ["id", "name", "Анализировать"];
        egui::Grid::new("articles").show(ui, |ui| {
            for label in header {
                ui.strong(label);
            }
            ui.end_row();
            for (i, article) in articles.iter() {
                ui.label(i.to_string());
                ui.label(option_to_string(article.name.as_ref()));
                let mut checked = chosen.contains(i);
                if ui.checkbox(&mut checked, "").changed() {
                    if checked {
                        chosen.insert(*i);
                    } else {
                        chosen.remove(i);
                    }
                }
                ui.end_row();
            }
        });
    }
    fn plot(ui: &mut egui::Ui, values: Points, size: f32) {
        egui_plot::Plot::new("Profit")
            .height(size)
            .clamp_grid(true)
            .x_axis_label("Время")
            .x_axis_formatter(|_, _| "".into())
            .y_axis_label("Деньги")
            .label_formatter(|_, point| {
                format!(
                    "Время: {}\nДеньги: {}",
                    DateTime::from_timestamp(point.x as i64, 0)
                        .map(|d| d.to_string())
                        .unwrap_or_default(),
                    point.y
                )
            })
            .show(ui, |plot_ui| {
                let debit_color = Color32::RED;
                let credit_color = Color32::BLUE;
                let debit = egui_plot::Line::new(
                    "Прибыль от времени",
                    PlotPoints::Borrowed(&values.debits),
                )
                .color(debit_color);
                let credit = egui_plot::Line::new(
                    "Расходы от времени",
                    PlotPoints::Borrowed(&values.credits),
                )
                .color(credit_color);
                plot_ui.line(debit);
                plot_ui.line(credit);
            });
    }
    fn set_err(&mut self, err: impl std::error::Error) {
        let message = format!("{err:?}");
        log::error!("{}", message);
        self.error_message = Some(message);
    }
}
