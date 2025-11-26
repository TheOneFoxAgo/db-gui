mod articles;
mod balance;
mod dynamics;
mod operations;
mod percents;
mod profit;

use std::borrow::Cow;

use strum::IntoStaticStr;

use crate::db::Db;

pub struct State {
    db: Db,
    selected: SelectedView,
    pub operations_state: operations::State,
}

pub enum Response {
    Exit,
    None,
}
#[derive(IntoStaticStr, Clone, Copy, PartialEq, Eq)]
pub enum SelectedView {
    #[strum(serialize = "Динамика")]
    Dynamics,
    #[strum(serialize = "Проценты")]
    Percentages,
    #[strum(serialize = "Прибыль")]
    Profit,
    #[strum(serialize = "Операции")]
    Operations,
    #[strum(serialize = "Статьи")]
    Articles,
    #[strum(serialize = "Баланс")]
    Balance,
    None,
}
impl Default for SelectedView {
    fn default() -> Self {
        Self::None
    }
}

impl State {
    pub fn new(db: Db) -> Self {
        Self {
            db,
            selected: SelectedView::None,
            operations_state: Default::default(),
        }
    }
    pub fn view(&mut self, ctx: &egui::Context) -> Response {
        let mut response = Response::None;
        egui::TopBottomPanel::top("Main page menu").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                egui::Sides::new().show(
                    ui,
                    // Слева
                    |ui| {
                        egui::widgets::global_theme_preference_buttons(ui);
                    },
                    // Справа
                    |ui| {
                        if ui.button("Выйти").clicked() {
                            response = Response::Exit;
                        }
                        ui.label(self.db.user());
                        ui.label("Пользователь:");
                        // Обратный порядок виджетов, из-за ограничений immediate-mode
                    },
                )
            });
        });
        egui::SidePanel::left("Tables").show(ctx, |ui| self.left_side(ui));
        egui::SidePanel::right("Indicators").show(ctx, |ui| self.right_side(ui));
        let placeholder = |ui: &mut egui::Ui| ui.heading("Not implemented");
        egui::CentralPanel::default().show(ctx, |ui| match &self.selected {
            SelectedView::Dynamics => {
                placeholder(ui);
            }
            SelectedView::Percentages => {
                placeholder(ui);
            }
            SelectedView::Profit => {
                placeholder(ui);
            }
            SelectedView::Operations => match self.operations_state.view(ui, &self.db, None) {
                operations::Response::ShowArticle(_id) => todo!(),
                operations::Response::ShowBalance(_id) => todo!(),
                operations::Response::None => {}
            },
            SelectedView::Articles => {
                placeholder(ui);
            }
            SelectedView::Balance => {
                placeholder(ui);
            }
            SelectedView::None => {
                placeholder(ui);
            }
        });
        return response;
    }
    fn left_side(&mut self, ui: &mut egui::Ui) {
        self.side_buttons(
            "Таблицы",
            &[
                SelectedView::Operations,
                SelectedView::Articles,
                SelectedView::Balance,
            ],
            ui,
        );
    }
    fn right_side(&mut self, ui: &mut egui::Ui) {
        self.side_buttons(
            "Индикаторы",
            &[
                SelectedView::Dynamics,
                SelectedView::Percentages,
                SelectedView::Profit,
            ],
            ui,
        );
    }
    fn side_buttons(&mut self, heading: &str, variants: &[SelectedView], ui: &mut egui::Ui) {
        ui.heading(heading);
        let mut chosen_one = SelectedView::None;
        for variant in variants {
            let enabled = self.selected == *variant;
            let label: &str = variant.into();
            if ui.selectable_label(enabled, label).clicked() {
                chosen_one = *variant;
            }
        }
        if chosen_one != SelectedView::None {
            self.selected = chosen_one;
        }
    }
}

pub fn option_to_string(option: Option<&impl ToString>) -> Cow<'_, str> {
    match option {
        Some(val) => Cow::Owned(val.to_string()),
        None => Cow::Borrowed(""),
    }
}
pub fn option_to_string_with<'a>(option: Option<&impl ToString>, default: &'a str) -> Cow<'a, str> {
    match option {
        Some(val) => Cow::Owned(val.to_string()),
        None => Cow::Borrowed(default),
    }
}
