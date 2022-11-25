pub fn process_path(path: String) -> String {
    if path != "." {
        if path.split(".").nth(1) == Some("png") {
            path
        } else {
            format!("{}{}", path, ".png")
        }
    } else {
        String::from("output.png")
    }
}
