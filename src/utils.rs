pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Join<T>
where
    Self: Iterator<Item = T>,
    T: std::fmt::Display,
{
    fn join(&mut self, joiner: &str) -> String;
}

impl<I, T> Join<T> for I
where
    I: Iterator<Item = T>,
    T: std::fmt::Display,
{
    fn join(&mut self, joiner: &str) -> String {
        let mut result = String::new();
        let mut first = true;

        for el in self {
            if first {
                result = format!("{}", &el);
                first = false;
            } else {
                result = format!("{}{}{}", result, joiner, &el)
            }
        }
        result
    }
}

pub trait One<T, F>
where
    Self: Iterator<Item = T>,
    F: Fn(&T) -> bool,
{
    fn one(self, predicate: F) -> Option<T>;
}

impl<I, T, F> One<T, F> for I
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> bool,
{
    fn one(self, predicate: F) -> Option<T> {
        for el in self {
            if predicate(&el) {
                return Some(el);
            }
        }
        None
    }
}
