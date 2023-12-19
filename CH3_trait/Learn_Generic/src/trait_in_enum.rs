pub enum _Options<T> {
    Some(T),
    None,
}

pub enum _Result<T,E> {
    Ok(T),
    Err(E),
}