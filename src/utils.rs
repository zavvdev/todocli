pub fn trim_str(target: &str) -> String {
    let entries: Vec<&str> = target.split_whitespace().collect();
    entries.join(" ")
}
