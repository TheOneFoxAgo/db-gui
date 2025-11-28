use crate::{
    app::{icons, main_page::option_to_string},
    db::scheme::ArticlesRow,
};
use std::collections::BTreeMap;
pub struct State {
    values: BTreeMap<i32, ArticlesRow>,
    edited: Option<(Option<i32>, ArticlesRow)>,
}
pub enum Response {
    Update(i32, ArticlesRow),
    Insert(ArticlesRow),
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
    pub fn new(values: BTreeMap<i32, ArticlesRow>) -> Self {
        Self {
            values,
            edited: None,
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui, edit_enabled: bool) -> Option<Response> {
        let mut response = None;
        let headers = ["id", "name", "Операции"];
        let regular_enabled = edit_enabled && self.edited.is_none();
        egui::containers::ScrollArea::new([true, true]).show(ui, |ui| {
            egui::Grid::new("Articles")
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
                            if let Some(inner_response) =
                                Self::show_edited_row(ui, Some(*target), edited_row, edit_enabled)
                            {
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
                                Self::show_normal_row(ui, *id, row, regular_enabled)
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
                            Self::show_edited_row(ui, None, edited_row, edit_enabled)
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
    pub fn inner(&self) -> &BTreeMap<i32, ArticlesRow> {
        &self.values
    }
    fn show_normal_row(
        ui: &mut egui::Ui,
        id: i32,
        row: &ArticlesRow,
        enabled: bool,
    ) -> Option<Regular> {
        ui.label(id.to_string());
        ui.label(option_to_string(row.name.as_ref()));
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
        edited_row: &mut ArticlesRow,
        enabled: bool,
    ) -> Option<Edited> {
        ui.label(option_to_string(id.as_ref()));
        ui.add_enabled(
            enabled,
            egui::TextEdit::singleline(edited_row.name.get_or_insert_default()),
        );

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
}
