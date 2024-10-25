pub trait Generator {
    /// Type of yielded value
    type Item;

    /// Yield next value if available
    fn next(&mut self) -> Option<Self::Item>;
}
