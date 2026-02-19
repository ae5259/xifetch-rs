use colored::{ColoredString, Colorize};


fn get_color() -> String {
    let file_path = format!(
        "/home/{}/.config/.xonfig",
        std::env::var("LOGNAME").unwrap_or(".config".to_owned())




    );

    let contents =
        std::fs::read_to_string(file_path).expect("Should have been able to read the file");

    let color = contents.split('=').collect::<Vec<&str>>()[1];

    String::from(color)
}

pub fn colorize(text: String) -> ColoredString {
    let string_color = get_color();

    let color = string_color.trim().to_lowercase();

    if color.trim() == "blue" {
        text.blue()
    } else if color == "red" {
        text.red()
    } else if color == "yellow" {
        text.yellow()
    } else if color == "black" {
        text.black()
    } else if color == "green" {
        text.green()
    } else if color == "purple" {
        text.purple()
    } else {
        text.white()
    }
}
