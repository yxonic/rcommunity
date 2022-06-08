use std::any::type_name;

#[must_use]
pub fn typename<T>() -> &'static str {
    let full_type_name = type_name::<T>();
    full_type_name.split("::").last().unwrap_or(full_type_name)
}
