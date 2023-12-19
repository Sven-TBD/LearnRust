mod cache;

fn main() {
    let use_closure = || {
        println!("this is a closure");
    };
    use_closure();
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x| x + 1;
    let add_one_v5 = |x| x + 1;

    let a = add_one_v2(5);
    let b = add_one_v3(5);
    let c = add_one_v4(5);
    let d = add_one_v5(5);

    println!("闭包不能推导两次");
    // let d = add_one_v5(5.1);

    // 捕捉变量
    let i = 1;
    let exe = |x| x + i;
    let r = exe(5);
    println!("{}", r);

    //create a cache
    let mut cache1 = cache::Cache::new(|x| x+1);
    let v1 = cache1.value(5);
    println!("value of cache is {}", v1);
    let v2 = cache1.value(3);
    println!("value of cache is {}", v2);

    // closure: get the variable in the context
    let x = 4;
    let equal_to_x = |z| z==x;
    let y = 4;
    assert!(equal_to_x(y));

    // 闭包有三种捕获其环境中变量的方式：获取所有权、可变借用、不可变借用
    // 对应三个trait
    // 1. FnOnce
    // 2. FnMut
    // 3. Fn

    let x = vec![1,2,4];
    let equal_to_x = move |z| {z==x};//x所有权移入闭包, x dropped at "}"
    // println!("{:?}",x);//此处必然报错 borrow of moved value: `x`
    // let equal_to_x = |z| z==x;
    let y = vec![1,2,4];
    assert!(equal_to_x(y)); 
    situation_1();
    situation_2_1();
    situation_2_2();
    situation_3_1();
}


fn situation_1(){
    //未捕获任何环境变量，自动实现Fn
    let c = || {println!("未捕获任何环境变量，自动实现Fn hhh")};
    c();
    c();
}

fn situation_2_1() {
    //如果捕获了Copy语义变量，且不需要修改它，则不管加不加 move，均会自动实现Fn
    let s = "hello";
    let c = ||{println!("如果捕获了Copy语义变量，且不需要修改它{}",s)};
    c();
    c();
    let c1 = move ||{println!("如果捕获了Copy语义变量，且不需要修改它 {}",s)};
    c1();
    c1();
}

fn situation_2_2() {
    //如果捕获了Copy语义变量，但需要修改它，则自动实现FnMut
    let mut s = 3;
    let c = ||{println!("如果捕获了Copy语义变量，but 需要修改它, FnMut by Auto {}",s+1)};
    c();
    c();
}

fn situation_3_1() {
    //如果捕获了Move语义变量，且不需要修改它，也没加 move，则实现FnOnce:
    let mut s = "hello".to_string();
    let mut c = || {
        s += " world";
        s
    };
    println!("FnOnce: {}", c());
    // c();
    // c();//this value implements `FnOnce`, which causes it to be moved when called
}

fn situation_3_2() {
    //如果捕获了Move语义变量，且不需要修改它，加了 move，同样实现FnOnce:
    let mut s = "hello".to_string();
    let mut c = move|| {
        s += " world";
        s
    };
    c();
    // c();
    // c();//this value implements `FnOnce`, which causes it to be moved when called
}

fn situation_3_3() {
    //如果捕获了Move语义变量，需要修改它，先实现FnMut:
    let mut s = "hello".to_string();
    let mut s2 = "hello".to_string();
    let mut c = move|| {
        s += " world";
        s
    };
    let mut c2 = || {
        s2 += " world";
        s2
    };
    c();
    // c();
    // c();//this value implements `FnOnce`, which causes it to be moved when called
}