use crate::my_traits::{GetInformation, SchoolName};

mod my_traits;
mod my_structs;

fn main() {

    //---
    let student_a = my_structs::Student::new("zsy",22);
    let student_a_school = student_a.get_school_name();
    println!("student a's name is {0},age is {1},School is {2}",student_a.get_name(), student_a.get_age(),student_a_school);
    print_info(student_a);
    let teacher_a_name = "vv";
    let teacher_a = my_structs::Teacher::new(teacher_a_name,22,"math");
    println!("teacher_a's name is {0},age is {1}",teacher_a.get_name(), teacher_a.get_age());
    print_info2(teacher_a);
    //---
    let student_b = produce_item_with_age("bb");
    print_info3(student_b);
}

fn print_info(item: impl my_traits::GetInformation) {
    println!("name = {}",item.get_name());
    println!("age = {}",item.get_age());
}


//trai bound 的写法1
fn print_info2<T:GetInformation>(item: T) {
    println!("name = {}",item.get_name());
    println!("age = {}",item.get_age());
}

//trai bound 的写法2
fn print_info3<T>(item: T)
    where T:GetInformation
{
    println!("name = {}",item.get_name());
    println!("age = {}",item.get_age());
}

fn produce_item_with_age(name:&str)-> impl GetInformation {
    my_structs::Student::new(name,10)
}