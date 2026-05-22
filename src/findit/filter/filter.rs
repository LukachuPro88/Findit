pub fn filter<T: AsRef<str>>(items: Vec<String>, target: T) -> Vec<String> {
    items
        .into_iter()
        .filter(|item| item.ends_with(target.as_ref()))
        .collect()
}
