import time
import sqlite3
from datetime import datetime
from AppKit import NSWorkspace
from Quartz import CGWindowListCopyWindowInfo, kCGWindowListOptionOnScreenOnly, kCGNullWindowID
import argparse

# Database configuration
DB_NAME = "active_app_usage.db"

def setup_database():
    """Create the database and table"""
    conn = sqlite3.connect(DB_NAME)
    cursor = conn.cursor()
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS app_usage (
            id INTEGER PRIMARY KEY,
            app_name TEXT NOT NULL,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            duration INTEGER NOT NULL  -- Recorded in seconds
        )
    """)
    conn.commit()
    conn.close()

def get_active_window_app():
    """Get the name of the application owning the currently active window"""
    options = kCGWindowListOptionOnScreenOnly
    window_list = CGWindowListCopyWindowInfo(options, kCGNullWindowID)

    for window in window_list:
        if window.get("kCGWindowLayer") == 0:  # Filter the topmost application window
            app_name = window.get("kCGWindowOwnerName", "Unknown")
            return app_name
    return "Unknown"

def log_app_usage(app_name, start_time, end_time, duration):
    """Log the application's usage time into the database (in seconds)"""
    conn = sqlite3.connect(DB_NAME)
    cursor = conn.cursor()
    cursor.execute("""
        INSERT INTO app_usage (app_name, start_time, end_time, duration)
        VALUES (?, ?, ?, ?)
    """, (app_name, start_time, end_time, duration))
    conn.commit()
    conn.close()

def track_active_app():
    """Track the active application and log its usage time into the database"""
    active_app = get_active_window_app()
    start_time = time.time()
    start_time_str = datetime.now().strftime('%Y-%m-%d %H:%M:%S')

    print("Tracking application usage. Press Ctrl+C to stop.")

    try:
        while True:
            current_app = get_active_window_app()
            if current_app != active_app:
                end_time = time.time()
                end_time_str = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
                duration_in_seconds = int(end_time - start_time)

                # Log usage time
                log_app_usage(active_app, start_time_str, end_time_str, duration_in_seconds)

                print(f"Logged: {active_app}, Duration: {duration_in_seconds} seconds")

                # Reset state
                active_app = current_app
                start_time = time.time()
                start_time_str = datetime.now().strftime('%Y-%m-%d %H:%M:%S')

            time.sleep(1)
    except KeyboardInterrupt:
        print("\nTracking stopped.")

def generate_daily_report():
    """Aggregate the daily application usage time"""
    conn = sqlite3.connect(DB_NAME)
    cursor = conn.cursor()
    cursor.execute("""
        SELECT 
            app_name, 
            DATE(start_time) as date, 
            SUM(duration) as total_duration
        FROM app_usage
        GROUP BY app_name, DATE(start_time)
        ORDER BY date, total_duration DESC
    """)
    report = cursor.fetchall()
    conn.close()

    print("Daily Report:")
    for app_name, date, total_duration in report:
        print(f"Date: {date}, App: {app_name}, Total Duration: {total_duration} seconds")

def main():
    """Process CLI arguments"""
    parser = argparse.ArgumentParser(description="Track and analyze active application usage on macOS.")
    parser.add_argument(
        "mode",
        choices=["track", "report"],
        help="Mode of operation: 'track' to record app usage, 'report' to generate daily report."
    )
    args = parser.parse_args()

    setup_database()

    if args.mode == "track":
        track_active_app()
    elif args.mode == "report":
        generate_daily_report()

if __name__ == "__main__":
    main()

