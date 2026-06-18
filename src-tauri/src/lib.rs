mod commands;
mod db;
mod error;
mod seed;
mod state;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app.path().app_local_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let db = db::open(&data_dir.join("data.db"))?;
            seed::seed_defaults(&db.lock())?;
            app.manage(AppState::new(db));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::goals::create_goal,
            commands::goals::list_goals,
            commands::goals::update_goal,
            commands::goals::archive_goal,
            commands::goals::delete_goal,
            commands::tasks::create_task,
            commands::tasks::list_tasks,
            commands::tasks::update_task,
            commands::tasks::complete_task,
            commands::tasks::delete_task,
            commands::tasks::reorder_tasks,
            commands::pomodoros::start_pomodoro,
            commands::pomodoros::complete_pomodoro,
            commands::pomodoros::interrupt_pomodoro,
            commands::pomodoros::list_pomodoros_today,
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::settings::get_all_settings,
            commands::stats::get_stats,
            commands::notify::notify_system,
            commands::notify::notify_sound,
            commands::notify::notify_vibration,
            commands::notify::set_foreground_timer,
            commands::notify::clear_foreground_timer,
            commands::notify::notify_focus_window,
            commands::notify::notify_taskbar_flash,
            commands::data::export_data,
            commands::data::import_data,
            commands::data::reset_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
