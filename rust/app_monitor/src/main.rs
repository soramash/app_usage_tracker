mod active_app;
mod db;

use chrono::Local;
use ctrlc;
use rusqlite::Result as RusqliteResult;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

fn track_active_app(db_name: &str) -> RusqliteResult<()> {
    let running = Arc::new(AtomicBool::new(true));
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }

    let mut active_app = active_app::get_active_window_app();
    let start_instant = Instant::now();
    let mut last_app = active_app.clone();
    let mut last_instant = start_instant;

    let mut current_start_time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    println!("Tracking application usage. Press Ctrl+C to stop.");

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(1));
        active_app = active_app::get_active_window_app();

        if active_app != last_app {
            let end_time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let duration_in_seconds = last_instant.elapsed().as_secs() as i64;

            db::log_app_usage(db_name, &last_app, &current_start_time_str, &end_time_str, duration_in_seconds)?;

            println!("Logged: {}, Duration: {} seconds", last_app, duration_in_seconds);

            last_app = active_app.clone();
            last_instant = Instant::now();
            current_start_time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        }
    }

    println!("\nTracking stopped.");
    Ok(())
}

fn main() -> RusqliteResult<()> {
    // ホームディレクトリを取得し、データベースパスを構築
    let db_path: PathBuf = home_dir()
        .expect("Could not determine home directory")
        .join("Library/Application Support/active_app_usage");

    // ディレクトリが存在しない場合は作成
    fs::create_dir_all(&db_path).expect("Failed to create database directory");

    // データベースファイルのパスを取得
    let db_name: PathBuf = db_path.join("active_app_usage.db");

    println!("Using database at: {}", db_name.display());

    // コマンドライン引数の処理
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [track|report]", args[0]);
        std::process::exit(1);
    }

    // データベースパスを文字列として取得
    let db_name_str = db_name
        .to_str()
        .expect("Failed to convert PathBuf to string");

    // データベースの初期設定
    db::setup_database(db_name_str)?;

    // モードごとの処理
    match args[1].as_str() {
        "track" => track_active_app(db_name_str)?,
        "report" => db::generate_daily_report(db_name_str)?,
        _ => {
            eprintln!("Unknown mode: {}. Use 'track' or 'report'.", args[1]);
            std::process::exit(1);
        }
    }

    Ok(())
}
