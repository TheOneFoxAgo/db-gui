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
    operations_state: operations::State,
    articles_state: articles::State,
    balance_state: balance::State,
    profit_state: profit::State,
    percents_state: percents::State,
    dynamics_state: dynamics::State,
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
}
impl State {
    pub fn new(db: Db) -> Self {
        Self {
            selected: SelectedView::Operations,
            operations_state: operations::State::new(&db),
            articles_state: articles::State::new(&db),
            balance_state: balance::State::new(&db),
            profit_state: profit::State::new(&db),
            percents_state: percents::State::new(&db),
            dynamics_state: dynamics::State::new(),
            db,
        }
    }
    pub fn view(&mut self, ctx: &egui::Context) -> Response {
        let mut response = Response::None;
        egui::TopBottomPanel::top("Main page menu").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
                ui.add_space(50.0);
                ui.label("Пользователь:");
                ui.label(self.db.user());
                if ui.button("Выйти").clicked() {
                    response = Response::Exit;
                }
            });
        });
        egui::SidePanel::left("Tables").show(ctx, |ui| {
            self.tables_selectors(ui);
            ui.add_space(20.0);
            self.indicator_selectors(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| match &self.selected {
            SelectedView::Dynamics => {
                self.dynamics_state
                    .view(ui, &self.db, self.articles_state.table());
            }
            SelectedView::Percentages => {
                self.percents_state.view(ui, &self.db);
            }
            SelectedView::Profit => {
                self.profit_state.view(ui, &self.db);
            }
            SelectedView::Operations => {
                self.operations_state
                    .view(ui, &self.db, self.articles_state.table())
            }
            SelectedView::Articles => self.articles_state.view(ui, &self.db),

            SelectedView::Balance => {
                self.balance_state.view(ui, &self.db);
            }
        });
        return response;
    }
    pub fn drive(&mut self) {
        self.operations_state.drive();
        self.articles_state.drive();
        self.balance_state.drive();
        self.profit_state.drive();
        self.percents_state.drive();
        self.dynamics_state.drive();
    }
    fn tables_selectors(&mut self, ui: &mut egui::Ui) {
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
    fn indicator_selectors(&mut self, ui: &mut egui::Ui) {
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
        let mut chosen_one = None;
        for variant in variants {
            let enabled = self.selected == *variant;
            let label: &str = variant.into();
            if ui.selectable_label(enabled, label).clicked() {
                chosen_one = Some(*variant);
            }
        }
        if let Some(chosen) = chosen_one {
            self.selected = chosen;
        }
    }
}

pub fn option_to_string(option: Option<&impl ToString>) -> String {
    option.map(|f| f.to_string()).unwrap_or_default()
}
pub fn option_to_string_with<'a>(option: Option<&impl ToString>, default: &'a str) -> Cow<'a, str> {
    match option {
        Some(val) => Cow::Owned(val.to_string()),
        None => Cow::Borrowed(default),
    }
}
