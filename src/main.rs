mod structtest;
use crate::structtest::{Coin, IpAddrKind, User, Book};
use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::sync::mpsc;
use std::time::Duration;
use std::{io, thread, vec};
use std::io::Read;

fn main() {
    let args = std::env::args();
    println!("{:?}", args);
    println!("Hello, world!");
    let c = add(3, 9);
    test17();
    println!("你好: {}", c);
}

fn test() {

    let a = 123;
    let mut b = 345;
    println!("{}",a);
    println!("{}",b);
    println!("=====");
    b = 990;
    println!("{}",a);
    println!("{}",b)
}

fn test2() {
    let x=32;
    let pi = 3.14;
    let is_true=false;
    let letter='a';
    
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn test3() {
    let number = 7;
    if number < 5 {
        println!("====1")
    } else {
        println!("====2")
    }

    let mut counter = 0;
    loop {
        counter += 1;
        if counter ==10 {
            println!("===break=== {}", counter);
            break;
        }
    }

    let mut delnumber = 3;
    while delnumber !=0 {
        delnumber -= 1;
    }

    for _number in 1..4  {
        println!("==={}", _number)
    }

    let s1 = String::from("hello");
    let s2 = s1.clone(); // 克隆一份以便后续同时使用
    println!("{}", s1);
    println!("{}", s2);

}


fn test4() {
    let user1 = User {
        username: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("===={:?}", user1);
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

}

fn value_in_cents(coin: Coin) ->u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Dime => 2,
        Coin::Nickel => 3,
        Coin::Quarter =>4,
    }
}

fn devide(a: i32, b: i32) -> Result<i32, String> {
    if  b==0 {
        Err(String::from("Devision by zero"))
    } else {
        Ok(a/b)
    }
}

fn test5() {
    // vec! 是 Rust 标准库提供的一个 宏（macro），用于方便地创建 Vec<T>（堆上可变长数组）
    // 以 ! 结尾是宏的标志
    let v = vec![1,2,3,4,5];
    // let mut iter = v.iter();
    // assert_eq!(iter.next(), Some(&1));
    // assert_eq!(iter.next(), Some(&2));
    // assert_eq!(iter.next(), Some(&3));
    // assert_eq!(iter.next(), Some(&4));
    // assert_eq!(iter.next(), Some(&5));
    // assert_eq!(iter.next(), None);

    let squred_vec: Vec<i32> = v.iter().map(|x| x*x).collect();
    println!("{:?}", squred_vec);
    let filter_vec: Vec<i32> = v.into_iter().filter(|&x| x%2==0).collect();
    println!("{:?}", filter_vec);
}

fn test6() {

    let vec = vec![1,2,3,4,5];
    for &num in vec.iter() {
        println!(" {} ", num);
    }
    println!("===🎶🎶🎶🎶===");
    let arr = [1,2,3,4,5];
    let mut iter = arr.into_iter().peekable();
    while let Some(val) = iter.next() {
        if val %2 == 0 {
            continue;
        }
        println!("{}", val);
    }
}

fn test7() {
    // 闭包，匿名函数
    let add = |a, b| a + b;
    println!("{}", add(96, 4));

    let x = 5;
    let squar = |num| num * x;
    println!("{}", squar(3));
}

fn test8() {
    // 按值，引用，可变借用捕获
    // 按值捕获，move，所有权被转移
    // 可变借用 mut， 闭包可以修改外部变量
    let mut num = 5;
    let print_num = || println!("{}", num);

    let take_num = move || println!("num taken = {}", num);
    // println!("{}", num) //报错， 所有权被转移

    let mut change_num = || num += 1;
    change_num();
    println!("num after closure = {}", num);
}

fn test9() {
    let book1 = Book::Paperty(31);
    let book2 = Book::Electronic(String::from("url://123.com"));

    if let Book::Electronic(url) = book2 {
        println!("==={}", url)
    } else {
        println!("===not electronic book")
    }
}

fn test10() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("file open success");
        },
        Err(error) => {
            println!("Failed to open file {}", error);
        }
    }
}

fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}

fn test11() {
    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => print!("{}",s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("no such file");
                },
                _ => {
                    println!("can not read the file");
                }
            }
        }


    }

}

fn test12() {
    let x = 5;          // x 的作用域开始
    let r = &x;         // r 的生命周期从这里开始
    println!("{}", r);  // 使用 r
} // x 的作用域结束 → x 被 drop → r 的生命周期必须在此前结束

fn test14() {
    // let vector: Vec<i32> = Vec::new();
    let mut vector = vec![1,2,3,4,5];
    vector.push(12);
    vector.push(13);
    println!("{:?}", vector);
    let mut v2: Vec<i32> = vec![16, 32, 64];
    vector.append(&mut v2);
    println!("{:?}", vector);

    let mut v = vec![100,32,57];
    for i in &mut v {
        *i += 50;
    }
    print!("{:?}", v);

}

fn test15() {
    let mut map = HashMap::new();
    map.insert("11", "old");
    map.insert("12", "new");
    println!("{:?}", map);
}

fn spawn_function() {
    for i in 0..5 {
        println!("spawned thread print {}", i);
        thread::sleep(Duration::from_secs(2));
    }
}

fn test16() {
    thread::spawn(spawn_function);
    for i in 0..7 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_secs(2));
    }
}

fn test17() {
    let(tx,rx) = mpsc::channel();
    thread::spawn(move ||{
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("=== received from {}", received);
}

fn test18() {
    let s = "hello";
    let handle = thread::spawn(move || {
        println!("{}", s);
    });
    handle.join().unwrap();
}