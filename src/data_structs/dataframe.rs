use std::vec::*;
// use rayon::prelude::*;
use std::fmt::Debug;
use std::ops::{Add, Div, Sub, Mul};
use num::{Zero, FromPrimitive};
use std::convert::*;

#[derive(Debug)]
pub struct DataFrame<T> {
    pub data: Vec<Vec<T>>,
    pub columns: Vec<String>
}

//mudar para a implementação pegar a coluna e não a linha
impl<T: Clone + 
    Debug + 
    Zero + 
    Copy + 
    FromPrimitive + 
    Add<T, Output = T> + 
    Div<T, Output = T> + 
    Sub<T, Output = T> + 
    Mul<T, Output = T> + 
    Into<f64>
    > DataFrame<T> {
    pub fn new() -> Self {
        DataFrame{
            data: Vec::with_capacity(0),
            columns: Vec::with_capacity(0)
        }
    }

    pub fn get_columns(&self) -> Vec<String> {
        self.columns.clone()
    }

    fn get_column_index(&self, column:String) -> usize {
        for i in 0..self.columns.len() {
            if self.columns[i] == column {
                return i
            }
        }
        usize::MAX
    }

    pub fn sum(&self, column: String) -> T {     
        self.data[self.get_column_index(column)].iter().fold(T::zero(), |sum, &val| sum + val)
    }

    pub fn mean(&self, column:String) -> T {
        let v = self.data[self.get_column_index(column.clone())].clone();
        self.sum(column.clone()) / FromPrimitive::from_usize(v.len()).unwrap()
    }

    pub fn min(&self, column: String) -> f64 {
        let v = self.data[self.get_column_index(column.clone())].clone();
        let mut min:f64 = f64::MAX;
        for x in 0..v.len() {
            if v[x].into() < min { 
                min = v[x].into();
            }
        }
        min
    }

    pub fn max(&self, column: String) -> f64 {
        let v = self.data[self.get_column_index(column.clone())].clone();
        let mut max : T  = T::zero();
        for x in 0..v.len() {
            if v[x].into() > max.into() { 
                max = v[x];
            }
        }
        max.into()
    }

    pub fn std(&self, column: String) -> f64 {
        let v = self.data[self.get_column_index(column.clone())].clone();
        let mut sum: f64 = 0.0;
        for i in 0..v.len() {
            sum = sum + (v[i] - self.mean(column.clone())).into().powf(2.0);
        }
        sum.sqrt()
    }

    pub fn head(&self) {
        if self.data.len() >= 3 {
            print!("  ");
            for i in 0..self.columns.len(){
                print!("{} ", self.columns[i]);
            }
            print!("\n");
            for i in 0..3 {
                println!("{:?} {:?}", i, self.data[i]);
            }
        }
    }

    pub fn describe(&self) {
        for i in 0..self.columns.len() - 1 {
            println!("Column: {}", self.columns[i]);
            println!("Min: {:?}", self.min(self.columns[i].clone()));
            println!("Max: {:?}", self.max(self.columns[i].clone()));
            println!("Sum: {:?}", self.sum(self.columns[i].clone()));
            println!("Mean: {:?}", self.mean(self.columns[i].clone()));
            println!("Std: {:?}", self.std(self.columns[i].clone()));
        }
    }


}