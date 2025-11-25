mod attribution;
mod login_page;
mod main_page;

pub enum App {
    Login(login_page::State),
    MainPage(main_page::State),
}
impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::Login(login_page::State::new())
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self {
            Self::Login(page) => {
                if let login_page::Response::SuccessfulLogin(db) = page.view(ctx) {
                    *self = Self::MainPage(main_page::State::new(db));
                }
            }
            Self::MainPage(page) => {
                page.view(ctx);
            }
        }
    }
}

macro_rules! drive_promise {
    ($promise:expr,
        Ok($res:ident) => $action:expr,
        Err($err:ident) => $handle:expr $(,)*
    ) => {
        if let Some(fulfilled) = $promise.take_if(|p| p.is_finished()) {
            match fulfilled.block_take() {
                Ok($res) => $action,
                Err($err) => $handle,
            }
        }
    };
}
pub(crate) use drive_promise;
macro_rules! drive_result_promise {
    ($promise:expr,
        Ok($res:ident) => $action:expr,
        Err($err:ident) => $handle:expr $(,)*
    ) => {
        crate::app::drive_promise!(
            $promise,
            Ok(res) => match res {
                Ok($res) => $action,
                Err($err) => $handle
            },
            Err($err) => $handle,
        )
    };
    ($promise:expr,
        Ok($res:ident) => $action:expr,
        Err($err:ident) => $handle:expr,
        JoinErr($jerr:ident) => $jhandle:expr $(,)*
    ) => {
        super::drive_promise!(
            $promise,
            Ok(res) => match res {
                Ok($res) => $action,
                Err($err) => $handle
            },
            Err($jerr) => $jhandle,
        )
    };
}
pub(crate) use drive_result_promise;
