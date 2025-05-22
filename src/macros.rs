#[macro_export]
macro_rules! add_filter {
    ($query:ident, $args:ident, $field:expr, $sql:expr) => {
        if let Some(val) = $field {
            $query.push_str($sql);
            let _ = $args.add(val);
        }
    };
}