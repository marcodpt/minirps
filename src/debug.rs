use chrono::Local;

pub fn debug (
    method: &str,
    path: &str,
    status: Option<u16>,
    error: &str
) -> () {
    println!("{} {} {} {}{}", 
        Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string(),
        method,
        path,
        match status {
            Some(status) => status.to_string(),
            None => String::from("...")
        },
        if error.len() > 0 {format!("\n{}", error)} else {String::new()}
    );
}
