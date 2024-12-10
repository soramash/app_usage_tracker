mod active_app;
mod db;
mod screen_lock;
use ctrlc;
use rusqlite::Result as RusqliteResult;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;


fn track_active_app(db_name: String, lock_state: Arc<Mutex<bool>>) -> RusqliteResult<()> {
    let running = Arc::new(AtomicBool::new(true));
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }

    let mut active_app = active_app::get_active_window_app();
    let start_instant = std::time::Instant::now();
    let mut last_app = active_app.clone();
    let mut last_instant = start_instant;
    let mut current_start_time_str = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    println!("Tracking application usage. Press Ctrl+C to stop.");

    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let is_locked = *lock_state.lock().unwrap();

        if is_locked {
            // ロック時の処理
            let end_time_str = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let duration_in_seconds = last_instant.elapsed().as_secs() as i64;
            db::log_app_usage(&db_name, &last_app, &current_start_time_str, &end_time_str, duration_in_seconds)?;
            println!("Screen locked. Logged: {}, Duration: {} seconds", last_app, duration_in_seconds);

            // ロック中は計測停止
            while *lock_state.lock().unwrap() && running.load(Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }

            if !running.load(Ordering::SeqCst) {
                break;
            }

            // ロック解除後、計測再開
            println!("Screen unlocked. Resuming application tracking.");
            active_app = active_app::get_active_window_app();
            last_app = active_app.clone();
            last_instant = std::time::Instant::now();
            current_start_time_str = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        } else {
            // ロックされていない場合、通常の計測処理
            active_app = active_app::get_active_window_app();
            if active_app != last_app {
                let end_time_str = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let duration_in_seconds = last_instant.elapsed().as_secs() as i64;

                db::log_app_usage(&db_name, &last_app, &current_start_time_str, &end_time_str, duration_in_seconds)?;
                println!("Logged: {}, Duration: {} seconds", last_app, duration_in_seconds);

                last_app = active_app.clone();
                last_instant = std::time::Instant::now();
                current_start_time_str = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            }
        }
    }

    println!("\nTracking stopped.");
    Ok(())
}



fn monitor_screen_lock(lock_state: Arc<Mutex<bool>>) {
    thread::spawn(move || {
        let mut last_state: Option<bool> = None;

        loop {
            let current_state = screen_lock::is_screen_locked();

            // Noneをunlocked(false)として扱う
            let interpreted_state = match current_state {
                Some(v) => v,
                None => false, // Noneはアンロック扱い
            };

            // last_state もOption<bool>だったが、今後はboolで比較した方がシンプル
            // よって、前回のbool値を保持するために別途変数を用意することを検討
            // ここでは last_bool_state を作成します。
            // 初回は last_state が None の場合は unlocked(false)とみなす
            let last_bool_state = last_state.unwrap_or(false);

            if interpreted_state != last_bool_state {
                if interpreted_state {
                    println!("Screen is now locked.");
                } else {
                    println!("Screen is now unlocked.");
                }

                // lock_stateを更新
                let mut locked_value = lock_state.lock().unwrap();
                *locked_value = interpreted_state;
            }

            // 今回の状態を更新
            last_state = Some(interpreted_state);

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}


fn main() -> RusqliteResult<()> {
    let db_path: PathBuf = home_dir()
        .expect("Could not determine home directory")
        .join("Library/Application Support/active_app_usage");

    fs::create_dir_all(&db_path).expect("Failed to create database directory");

    let db_name: PathBuf = db_path.join("active_app_usage.db");

    println!("Using database at: {}", db_name.display());

    let db_name_str = db_name
        .to_str()
        .expect("Failed to convert PathBuf to string")
        .to_string();

    db::setup_database(&db_name_str)?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [track|report]", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "track" => {
            let lock_state = Arc::new(Mutex::new(false));
            monitor_screen_lock(lock_state.clone());
            track_active_app(db_name_str, lock_state)?;
        }
        "report" => {
            db::generate_daily_report(&db_name_str)?;
        }
        _ => {
            eprintln!("Unknown mode: {}. Use 'track' or 'report'.", args[1]);
            std::process::exit(1);
        }
    }

    Ok(())
}
