use rusqlite::{params, Connection, Result as RusqliteResult};

pub fn setup_database(db_name: &str) -> RusqliteResult<()> {
    let conn = Connection::open(db_name)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS app_usage (
            id INTEGER PRIMARY KEY,
            app_name TEXT NOT NULL,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            duration INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn log_app_usage(db_name: &str, app_name: &str, start_time_str: &str, end_time_str: &str, duration: i64) -> RusqliteResult<()> {
    let conn = Connection::open(db_name)?;
    conn.execute(
        "INSERT INTO app_usage (app_name, start_time, end_time, duration) VALUES (?1, ?2, ?3, ?4)",
        params![app_name, start_time_str, end_time_str, duration],
    )?;
    Ok(())
}

pub fn generate_daily_report(db_name: &str) -> RusqliteResult<()> {
    let conn = Connection::open(db_name)?;
    let mut stmt = conn.prepare(
        "SELECT 
            app_name,
            DATE(start_time) as date,
            SUM(duration) as total_duration
        FROM app_usage
        GROUP BY app_name, DATE(start_time)
        ORDER BY date, total_duration DESC"
    )?;

    let report_iter = stmt.query_map([], |row| {
        let app_name: String = row.get(0)?;
        let date: String = row.get(1)?;
        let total_duration: i64 = row.get(2)?;
        Ok((app_name, date, total_duration))
    })?;

    println!("Daily Report:");
    for result in report_iter {
        let (app_name, date, total_duration) = result?;
        println!("Date: {}, App: {}, Total Duration: {} seconds", date, app_name, total_duration);
    }

    Ok(())
}
