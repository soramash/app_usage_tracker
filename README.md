# app_usage_tracker
Tracking tools to monitor the active application usage on macOS

# Active Application Usage Tracker

This tool is designed to track the active application usage on macOS and store the data in a SQLite database. It supports two modes of operation:
1. **Tracking Mode**: Logs active application usage in real-time.
2. **Report Mode**: Generates a daily report summarizing the total usage time for each application in minutes.

## Features
- Real-time tracking of active applications on macOS.
- Logs application usage in minute increments.
- Provides daily usage summaries.

---

## Prerequisites

### Requirements
- macOS operating system.
- Python 3.6 or later.
- Required Python libraries:
  - `pyobjc`

### Installation of `pyobjc`
```bash
pip install pyobjc
```

### Accessibility Permissions
To allow the tool to access information about active windows, ensure the Python interpreter or your terminal application has Accessibility permissions:

1. Go to **System Preferences** > **Security & Privacy** > **Privacy**.
2. Select **Accessibility**.
3. Add your terminal application (e.g., Terminal, iTerm, or VSCode) to the list.
4. Restart the terminal application if needed.

---

## Installation

1. Clone or download this repository.
2. Ensure Python and `pyobjc` are installed.
3. Grant the required permissions (as mentioned above).

---

## Usage

Run the script in one of two modes: **Tracking** or **Reporting**.

### Tracking Mode
Track active applications and log their usage to the database in real-time.
```bash
python app_usage_tracker.py track
```
**Example Output:**
```
Tracking application usage. Press Ctrl+C to stop.
Logged: Safari, Duration: 3 minutes
Logged: Terminal, Duration: 2 minutes
```

### Report Mode
Generate a daily summary of application usage.
```bash
python app_usage_tracker.py report
```
**Example Output:**
```
Daily Report:
Date: 2024-12-06, App: Safari, Total Duration: 25 minutes
Date: 2024-12-06, App: Terminal, Total Duration: 15 minutes
```

---

## How It Works

1. **Tracking Mode**:
   - Uses macOS Accessibility API to determine the active application.
   - Records the application's name, start time, end time, and duration (in minutes) into a SQLite database (`active_app_usage.db`).

2. **Report Mode**:
   - Reads the logged data from the database.
   - Aggregates the total usage time for each application by day.

---

## Database Schema
The SQLite database (`active_app_usage.db`) contains the following table:

### `app_usage`
| Column      | Type    | Description                           |
|-------------|---------|---------------------------------------|
| `id`        | INTEGER | Primary key                          |
| `app_name`  | TEXT    | Name of the application               |
| `start_time`| TEXT    | Start time of the application session |
| `end_time`  | TEXT    | End time of the application session   |
| `duration`  | INTEGER | Duration of the session in minutes    |

---

## Limitations
- Only tracks the application currently in focus.
- Requires Accessibility permissions to function.

---

## Potential Enhancements
- Support for exporting reports to CSV or JSON.
- Visualization of usage data (e.g., pie charts or graphs).
- Multi-device data aggregation.

---

## License
This tool is licensed under the MIT License. See the LICENSE file for details.

