use super::{attribution, drive_result_promise};
use crate::{db::Db, promise_lite::PromiseLite};
use std::cell::LazyCell;
use tokio_postgres::Error;

pub struct State {
    user: String,
    password: String,
    error_message: Option<String>,
    result: Option<PromiseLite<Result<Db, Error>>>,
}
pub enum Response {
    SuccessfulLogin(Db),
    None,
}
impl State {
    pub fn new() -> Self {
        Self {
            user: "".into(),
            password: "".into(),
            error_message: None,
            result: None,
        }
    }
    pub fn view(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("Login page menu").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Авторизация");
            let (user, password) = egui::Grid::new("User and password")
                .num_columns(2)
                .show(ui, |ui| {
                    // Ввод логина
                    ui.label("Пользователь:");
                    let user = ui.text_edit_singleline(&mut self.user);
                    ui.end_row();

                    // Ввод пароля
                    ui.label("Пароль:");
                    let password =
                        ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                    (user, password)
                })
                .inner;

            // Кнопка авторизации
            let (button_text, enabled) = match self.authorization_status() {
                Status::Ok => ("Авторизоваться!", true),
                Status::Waiting => ("Ожидайте...", false),
                Status::Repeat => ("Повторите попытку!", true),
            };
            let authorize_button = ui.add_enabled(enabled, egui::Button::new(button_text));
            self.handle_focus(&user, &password, &authorize_button, ui);
            if authorize_button.clicked() {
                log::info!(
                    "Попытка авторизации. Логин: {}, Пароль {}",
                    self.user,
                    self.password
                );
                self.result = Some(Db::new(
                    self.user.clone(),
                    self.password.clone(),
                    ctx.clone(),
                ));
            }

            // Сообщение об ошибке
            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, error);
            }

            // Уважаем разработчиков
            attribution::pay_respect(ui);
        });
    }
    pub fn drive(&mut self) -> Response {
        drive_result_promise!(
            self.result,
            Ok(db) => return Response::SuccessfulLogin(db),
            Err(err) => {
                let message = format!("{err:?}");
                log::error!("{}", message);
                self.error_message = Some(message);
            },
        );
        Response::None
    }
    fn handle_focus(
        &mut self,
        user: &egui::Response,
        password: &egui::Response,
        authorize: &egui::Response,
        ui: &mut egui::Ui,
    ) {
        let enter_pressed = LazyCell::new(|| ui.input(|i| i.key_pressed(egui::Key::Enter)));
        if user.lost_focus() && *enter_pressed {
            log::info!("Ввели пользователя: {}", self.password);
            password.request_focus();
            return;
        }
        if password.lost_focus() && *enter_pressed {
            log::info!("Ввели пароль: {}", self.password);
            authorize.request_focus();
        }
    }
    fn authorization_status(&self) -> Status {
        if self.result.is_none() {
            if self.error_message.is_none() {
                Status::Ok
            } else {
                Status::Repeat
            }
        } else {
            Status::Waiting
        }
    }
}
enum Status {
    Ok,
    Waiting,
    Repeat,
}
