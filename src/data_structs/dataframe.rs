use std::vec::*;

#[derive(Debug)]
pub struct DataFrame<T> {
    pub data: Vec<Vec<T>>,
    pub columns: Vec<String>
}

impl<T: Clone> DataFrame<T> {
    pub fn new() -> Self {
        DataFrame{
            data: Vec::with_capacity(0),
            columns: Vec::with_capacity(0)
        }
    }

    pub fn get_columns(&self) -> Vec<String> {
        self.columns.clone()
    }

}