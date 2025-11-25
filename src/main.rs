#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::Arc;

use anyhow;
use tokio;

fn main() -> anyhow::Result<()> {
    // Отправляем логи в stderr
    // `RUST_LOG=debug`
    env_logger::init();

    // Разворачиваем асинхронный рантайм, для работы с базой.
    // Нам хватит однопоточной версии.
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()?;

    // Входим в контекст рантайма. Теперь мы можем спавнить таски.
    let _rt_guard = rt.enter();

    // Сигналы завершения работы
    let exit = std::sync::Arc::new(tokio::sync::Notify::new());
    let guard = exit.clone();

    // Рантайм будет работать в отдельном потоке, чтобы не блокировать ui
    std::thread::spawn(move || {
        rt.block_on(async move {
            exit.notified().await;
            log::info!("Рантайм закончил свою работу");
        })
    });

    // Автоматически завершаем работу рантайма, когда выходим
    let _guard = ExitGuard(guard);

    // Настраиваем окошко
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    // Запускаем основной цикл приложения
    if let Err(err) = eframe::run_native(
        "db-gui",
        native_options,
        Box::new(|cc| Ok(Box::new(dbgui::App::new(cc)))),
    ) {
        anyhow::bail!("Eframe failed: {}", err)
    }
    Ok(())
}

struct ExitGuard(Arc<tokio::sync::Notify>);
impl Drop for ExitGuard {
    fn drop(&mut self) {
        self.0.notify_one();
    }
}
