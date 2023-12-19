mod largest;
mod trait_in_struct;
mod trait_in_enum;
fn main() {
    let number_list = vec![1, 2, 7, 4, 10, 6];
    let max_number = largest::largest_i32(&number_list);
    println!("max number is :{0}",max_number);

    let char_list = vec!['z','b','c','0','😅'];
    let max_char = largest::largest_char(&char_list);
    println!("max char is :{0}",max_char);

    //泛型函数
    let max_number2 = largest::largest(&number_list);
    println!("max number2 is :{0}",max_number2);
    
    let char_list2 = largest::largest::<char>(&char_list);
    println!("max char2 is :{0}",char_list2);

    //结构体泛型
    let integer = trait_in_struct::Point1{x:1,y:1};
    println!("integer Point is :{}",integer);

    let point_a = trait_in_struct::Point2::new(1, 'b');
    println!("{:#?}",point_a);

    //方法泛型
    let point_b = trait_in_struct::Point3::new(1,1);
    let point_b_x = point_b.get_x();
    println!("the x is of point_b_x is :{0}",point_b_x);


}
