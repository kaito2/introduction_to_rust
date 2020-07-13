pub mod first;
pub mod second;

// SortOrderを列挙型として定義する
pub enum SortOrder {
    // SortOrder には2つのバリアントがある
    Ascending,  // 昇順
    Descending, // 降順
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
