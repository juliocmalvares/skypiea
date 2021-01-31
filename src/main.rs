
pub mod data_structs;

fn main() {
    println!("Hello World");
    let mut data = data_structs::dataframe::DataFrame::<i32>::new();
    data.data.push(vec![0,1,2, 3]);
    data.data.push(vec![3,4,5, 6]);
    data.data.push(vec![6,7,8, 9]);
    println!("{:?}", data.data);
    data.columns.push(String::from("c1"));
    data.columns.push(String::from("c2"));
    data.columns.push(String::from("c3"));
    data.columns.push(String::from("c4"));
    println!("{:?}", data.get_columns());
    println!("Sum: {:?}", data.sum(String::from("c2")));
    println!("Mean: {:?}", data.mean(String::from("c2")));
    println!("Std: {:?}", data.std(String::from("c2")));
    data.head();

    println!("\n\n");
    data.describe();
}

