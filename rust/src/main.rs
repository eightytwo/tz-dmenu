use std::fs;
use std::path::PathBuf;

use chrono::DateTime;
use chrono::Local;
use chrono_tz::Tz;
use dirs;
use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::Redirection;
use toml::Value;

fn load_config(config_file: PathBuf) -> Vec<(String, DateTime<Tz>)> {
    // Read the configuration file into a string
    let contents = fs::read_to_string(config_file).expect(
        "Something went wrong reading the config file. \
         Please ensure it exists at ~/.config/tz_dmenu/config.toml",
    );

    // Parse the configuration string as a toml value
    let toml_value = contents.parse::<Value>().unwrap();

    // Get the date and time in the local time zone and prepare a vector
    // for storing the current time in the configured time zones.
    let local_now: DateTime<Local> = Local::now();
    let mut results: Vec<(String, DateTime<Tz>)> = Vec::new();

    // Read the time zones from the configuration and record the human
    // time zone name along with the local time in that time zone.
    for (human_name, tz_name) in toml_value["timezones"].as_table().unwrap() {
        let tz_name: String = tz_name.as_str().unwrap().to_string();
        let tz: Tz = tz_name.parse().unwrap();
        results.push((human_name.to_string(), local_now.with_timezone(&tz)));
    }

    results
}

fn build_entries(tzs: Vec<(String, DateTime<Tz>)>) -> String {
    let mut entries: String = String::new();

    // Determine the length of the longest location name
    let max_chars = tzs
        .iter()
        .max_by_key(|v| v.0.chars().count())
        .unwrap()
        .0
        .chars()
        .count();

    // Append each time zone, along with the current time in that time zone, as
    // a separate line to the entries string.
    for (tz_name, dt) in tzs {
        let padding = max_chars - tz_name.len() + 1;
        let mut entry: String = String::from(tz_name);
        entry.push_str(&" ".repeat(padding));
        entry.push_str(&dt.format("%l:%M %p %z").to_string());
        entry.push_str("\n");
        entries.push_str(&entry);
    }

    entries
}

fn run_dmenu(entries: String) {
    // Count the number of lines in the string
    let num_lines: usize = entries.lines().collect::<Vec<&str>>().len();

    // Create the dmenu child process
    let p = Popen::create(
        &["dmenu", "-i", "-l", &num_lines.to_string(), "-p", ">"],
        PopenConfig {
            stdout: Redirection::Pipe,
            stdin: Redirection::Pipe,
            ..Default::default()
        },
    );

    // Feed the time zones string to dmenu
    p.unwrap().communicate(Some(&entries)).unwrap();
}

fn main() {
    // Build the file path to the configuration file
    let config_file: PathBuf = dirs::config_dir()
        .unwrap()
        .join("tz_dmenu")
        .join("config.toml");

    // Load the time zones in the config file and represent them as a Vector of
    // name and datetime pairs.
    let mut tzs: Vec<(String, DateTime<Tz>)> = load_config(config_file);

    // Sort the time zones from largest offset to smallest offset
    tzs.sort_by(|a, b| a.1.to_string().cmp(&b.1.to_string()).reverse());

    // Bundle all time zones (and the current time in each time zone) into
    // a single string, with one line per time zone.
    let entries: String = build_entries(tzs);

    // Display the time zones via dmenu
    run_dmenu(entries);
}
