use chrono::{Local, DateTime};

pub fn time_string (time: DateTime<Local>) -> String {
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn debug (
    method: &str,
    path: &str,
    status: Option<u16>,
    error: &str
) -> () {
    println!("[{}] {} {} {}{}", 
        time_string(Local::now()),
        method,
        path,
        match status {
            Some(status) => status.to_string(),
            None => String::from("...")
        },
        if error.len() > 0 {format!("\n{}", error)} else {String::new()}
    );
}
