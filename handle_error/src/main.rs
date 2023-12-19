use std::{fs::File, io::Read};
mod error_transfer;
fn main() {
    ////way 1
    // let f = File::open("hello.txt");
    // let mut f = match f{
    //     Ok(file) => file,
    //     Err(err) => panic!("error: {:?}", err),
    // };

    ////way 2
    // let mut f = File::open("hello.txt").unwrap();

    //way 3
    let mut f = File::open("hello.txt").expect("failed");
    print_file(&mut f);
    
    let mut filename = "hello.txt";
    
    let contents1 = error_transfer::read_username_from_file1(&mut filename);
    let contents2 = error_transfer::read_username_from_file2(&mut filename);
    let contents3 = error_transfer::read_username_from_file3(&mut filename);

    println!("{0}\n,{1}\n,{2}", contents1.unwrap(), contents2.unwrap(), contents3.unwrap());
}

fn print_file(file: &mut File) {
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    println!("内容如下q: \n{}", contents);
}
