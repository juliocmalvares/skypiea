
pub mod data_structs;

fn main() {
    println!("Hello World");
    let mut data = data_structs::dataframe::DataFrame::<i32>::new();
    data.data.push(vec![0,1,2]);
    data.data.push(vec![3,4,5]);
    data.data.push(vec![6,7,8]);
    println!("{:?}", data.data);
    data.columns.push(String::from("c1"));
    data.columns.push(String::from("c2"));
    data.columns.push(String::from("c3"));
    println!("{:?}", data.get_columns());
    println!("Sum: {:?}", data.sum(String::from("c2")));
    println!("Mean: {:?}", data.mean(String::from("c2")));
    println!("Std: {:?}", data.std(String::from("c2")));
}

