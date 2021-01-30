
pub mod data_structs;

fn main() {
    println!("Hello World");
    let mut data = data_structs::dataframe::DataFrame::<i32>::new();
    data.data.push(vec![0,1,2]);
    data.data.push(vec![3,4,5]);
    data.data.push(vec![6,7,8]);
    data.data[0].push(10);
    println!("{:?}", data.data);
    data.columns.push(String::from("Teste"));
    println!("{:?}", data.get_columns());
}

