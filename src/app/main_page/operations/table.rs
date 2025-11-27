use crate::{
    app::{
        icons,
        main_page::{option_to_string, option_to_string_with},
    },
    db::scheme::{ArticlesRow, OperationsRow},
};
use std::collections::BTreeMap;
pub struct State {
    values: BTreeMap<i32, OperationsRow>,
    edited: Option<(Option<i32>, OperationsRow)>,
}
pub enum Response {
    Update(i32, OperationsRow),
    Insert(OperationsRow),
    Delete(i32),
}
enum Edited {
    Confirm,
    Cancel,
}
enum Regular {
    Edit,
    Delete,
}
impl State {
    pub fn new(values: BTreeMap<i32, OperationsRow>) -> Self {
        Self {
            values,
            edited: None,
        }
    }
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        edit_enabled: bool,
        articles: &BTreeMap<i32, ArticlesRow>,
    ) -> Option<Response> {
        let mut response = None;
        let headers = [
            "id",
            "article_id",
            "debit",
            "credit",
            "create_date",
            "balance_id",
            "Операции",
        ];
        let regular_enabled = edit_enabled && self.edited.is_none();
        egui::containers::ScrollArea::new([true, true]).show(ui, |ui| {
            egui::Grid::new("Operations")
                .num_columns(headers.len())
                .show(ui, |ui| {
                    for header in headers {
                        ui.strong(header);
                    }
                    ui.end_row();
                    for (id, row) in self.values.iter() {
                        if let Some((Some(target), edited_row)) = &mut self.edited
                            && *target == *id
                        {
                            if let Some(inner_response) = Self::show_edited_row(
                                ui,
                                Some(*target),
                                edited_row,
                                edit_enabled,
                                articles,
                            ) {
                                match inner_response {
                                    Edited::Confirm => {
                                        response = Some(Response::Update(*id, edited_row.clone()))
                                    }
                                    Edited::Cancel => {
                                        self.edited = None;
                                    }
                                }
                            }
                        } else {
                            if let Some(inner_response) =
                                Self::show_normal_row(ui, *id, row, regular_enabled, articles)
                            {
                                match inner_response {
                                    Regular::Edit => self.edited = Some((Some(*id), row.clone())),
                                    Regular::Delete => response = Some(Response::Delete(*id)),
                                }
                            }
                        }
                        ui.end_row();
                    }
                    if let Some((None, edited_row)) = &mut self.edited {
                        if let Some(inner_response) =
                            Self::show_edited_row(ui, None, edited_row, edit_enabled, articles)
                        {
                            match inner_response {
                                Edited::Confirm => {
                                    response = Some(Response::Insert(edited_row.clone()))
                                }
                                Edited::Cancel => {
                                    self.edited = None;
                                }
                            }
                        }
                    }
                });
        });
        response
    }
    pub fn insert_new_row(&mut self) {
        self.edited = Some((None, Default::default()));
    }
    pub fn is_changing(&self) -> bool {
        self.edited.is_some()
    }
    fn show_normal_row(
        ui: &mut egui::Ui,
        id: i32,
        row: &OperationsRow,
        enabled: bool,
        articles: &BTreeMap<i32, ArticlesRow>,
    ) -> Option<Regular> {
        ui.label(id.to_string());
        ui.label(Self::format_from_articles(Some(id), articles));
        ui.label(option_to_string(row.debit.as_ref()));
        ui.label(option_to_string(row.credit.as_ref()));
        ui.label(option_to_string(row.create_date.as_ref()));
        ui.label(option_to_string_with(row.balance_id.as_ref(), "[null]"));
        let mut response = None;
        ui.horizontal(|ui| {
            let edit = egui::Button::new(icons::EDIT).small();
            let remove = egui::Button::new(icons::REMOVE).small();
            if ui.add_enabled(enabled, edit).clicked() {
                response = Some(Regular::Edit);
            }
            if ui.add_enabled(enabled, remove).clicked() {
                response = Some(Regular::Delete);
            }
        });
        response
    }
    fn show_edited_row(
        ui: &mut egui::Ui,
        id: Option<i32>,
        edited_row: &mut OperationsRow,
        enabled: bool,
        articles: &BTreeMap<i32, ArticlesRow>,
    ) -> Option<Edited> {
        ui.label(option_to_string(id.as_ref()));
        egui::ComboBox::from_id_salt("choose article")
            .selected_text(Self::format_from_articles(edited_row.article_id, articles))
            .show_ui(ui, |ui| {
                for (id, article) in articles {
                    ui.selectable_value(
                        &mut edited_row.article_id,
                        Some(*id),
                        Self::format_article(*id, article),
                    );
                }
            });
        let mut debit = edited_row.debit.unwrap_or(0);
        if ui
            .add(egui::DragValue::new(&mut debit).speed(0.5))
            .changed()
        {
            edited_row.debit = Some(debit);
        }

        let mut credit = edited_row.credit.unwrap_or(0);
        if ui
            .add(egui::DragValue::new(&mut credit).speed(0.5))
            .changed()
        {
            edited_row.credit = Some(credit);
        }

        let mut create_date = edited_row.create_date.map(|t| t.date()).unwrap_or_default();
        if ui
            .add(egui_extras::DatePickerButton::new(&mut create_date))
            .changed()
        {
            edited_row.create_date = Some(create_date.into());
        }

        ui.label(option_to_string_with(
            edited_row.balance_id.as_ref(),
            "[null]",
        ));
        let mut response = None;
        ui.horizontal(|ui| {
            let confirm = egui::Button::new(icons::CONFIRM).small();
            let cancel = egui::Button::new(icons::CANCEL).small();
            if ui.add_enabled(enabled, confirm).clicked() {
                response = Some(Edited::Confirm);
            }
            if ui.add_enabled(enabled, cancel).clicked() {
                response = Some(Edited::Cancel);
            }
        });
        response
    }
    fn format_from_articles(id: Option<i32>, articles: &BTreeMap<i32, ArticlesRow>) -> String {
        if let Some(id) = id {
            if let Some(article) = articles.get(&id) {
                return Self::format_article(id, article);
            }
        }
        "".into()
    }
    fn format_article(id: i32, article: &ArticlesRow) -> String {
        format!("{id} ({})", option_to_string(article.name.as_ref()))
    }
}
