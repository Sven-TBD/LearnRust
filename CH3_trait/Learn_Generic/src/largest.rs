pub fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() { //这里 item前加 & 是对应模式匹配
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() { //这里 item前加 & 是对应模式匹配
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 泛型函数
pub fn largest<T:PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item> largest{
            largest = item;
        }
    }
    largest
}