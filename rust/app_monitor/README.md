# App Monitor

A macOS application usage tracker written in Rust. It records the active application's usage time in a SQLite database and generates daily reports.

## Features

- **Track active application usage:** Continuously monitors the currently active application and logs its usage time.
- **Generate daily reports:** Provides a summary of application usage for each day.
- **Automatic start at login:** Can be configured to start automatically on macOS login.
- **Database management:** Stores data securely in a SQLite database located in the user's library folder.

## Installation

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/app_monitor.git
cd app_monitor
```

### 2. Build the project
```bash
cargo build --release
```
The compiled binary will be located in the target/release/ directory.

### 3. Install the binary
Copy the binary to a directory in your PATH, such as /usr/local/bin:

```bash
sudo cp target/release/app_monitor /usr/local/bin
```

## Usage
### Tracking Application Usage
Run the program in track mode to monitor active application usage:

```bash
app_monitor track
```
The data will be saved in the SQLite database located at:

```javascript
~/Library/Application Support/active_app_usage/active_app_usage.db
```

### Generating Daily Reports
Run the program in report mode to view the daily usage summary:

```bash
app_monitor report
```

### Configure Automatic Start at Login
To automatically start tracking when the system boots:

1. Create a LaunchAgent plist file at `~/Library/LaunchAgents/com.example.appmonitor.plist` with the following content:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.example.appmonitor</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/app_monitor</string>
        <string>track</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

2. Move the file to the `LaunchAgents` directory:

```bash
mv com.example.appmonitor.plist ~/Library/LaunchAgents/
```
3. Load the LaunchAgent:

```bash
launchctl load ~/Library/LaunchAgents/com.example.appmonitor.plist
```

4. Add code signing (if you put the app under the system directory such as /usr/local/bin)

```bash
codesign --force --deep --sign - /usr/local/bin/app_monitor
```

## Requirements
- macOS
- Rust (for building)
- SQLite


## Development
### Prerequisites

- Install Rust: Rust Official Installation Guide

### Run in Debug Mode
To run the application in debug mode:

```bash
cargo run track
```

### Code Structure
- main.rs: Handles the CLI and program flow.
- active_app.rs: Contains the logic to retrieve the active application name.
- db.rs: Manages database operations, including tracking and reporting.

## License
This project is licensed under the MIT License. See the LICENSE file for details.
