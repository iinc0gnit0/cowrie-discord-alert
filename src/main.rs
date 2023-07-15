use logwatcher::{LogWatcher, LogWatcherAction};
use serde_json::{from_str, Value};

fn main() {
    // Change these 2 values
    let log_file_path: &str = "/path/to/log/file/var/log/cowrie/cowrie.json";
    let discord_webhook: &str = "https://discord.com/api/webhooks/";

    let mut log_watcher = LogWatcher::register(log_file_path).unwrap();
    log_watcher.watch(&mut move |line: String| {
        let data: Value = match from_str(&line) {
            Ok(v) => v,
            Err(_) => Value::Null,
        };
        if data["eventid"] == "cowrie.login.success" {
            println!("{line}");
            let content = format!("```---------------\neventid: {}\nusername: {}\npassword: {}\nmessage: {}\nsensor: {}\ntimestamp: {}\nsrc_ip: {}\nsession: {}```", data["eventid"], data["username"], data["password"], data["message"], data["sensor"], data["timestamp"], data["src_ip"], data["session"]);

            ureq::post(discord_webhook)
                .set("Content-Type", "application/json")
                .send_json(ureq::json!({
                    "content": content,
                })).unwrap().into_string().unwrap();

        } else if data["eventid"] == "cowrie.command.input" {
            println!("{line}");
            let content = format!("```---------------\neventid: {}\ninput: {}\nmessage: {}\nsensor: {}\ntimestamp: {}\nsrc_ip: {}\nsession: {}```", data["eventid"], data["input"], data["message"], data["sensor"], data["timestamp"], data["src_ip"], data["session"]);

            ureq::post(discord_webhook)
                .set("Content-Type", "application/json")
                .send_json(ureq::json!({
                    "content": content,
                })).unwrap().into_string().unwrap();
        }
        LogWatcherAction::None
    });
}
