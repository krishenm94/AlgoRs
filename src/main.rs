mod b1;

fn main(){
    println!("Hello world!");

    let mut vec: Vec<i32> = vec![89, 3, 5, 34, 8, 1, 13, 21, 55, 2, 1];
    b1::ch1::sort::insertion(vec);
}