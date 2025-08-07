// Entry-Enum


// use std::collections::HashMap;
// fn tut(vec: Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
//     let mut map: HashMap<String, Vec<i32>> = HashMap::new();
//     for (key, value) in vec {
//         map.entry(key).or_insert(vec![]).push(value);
//     }
//     map
// }


// fn main(){
//    let mut vec: Vec<(String, i32)> = vec![
//         ("a".to_string(), 1),
//         ("b".to_string(), 2),
//         ("a".to_string(), 3),
//         ("c".to_string(), 4),
//         ("b".to_string(), 5),
//     ];
//     let map = tut(vec);
//     for (key, value) in map {
//         println!("{}: {:?}", key, value);
//     }

// }


// LEarning lifetime
// lifetime generic annotation
// lifettime of return type = intersectios of lifetime of parameters
//fn tut2<'a>(a: &'a String, b: &'a String)-> &'a String{
    // if a.chars().count() > b.chars().count(){
    //     &*a
    // }else{
    //     &*b
    // }
//     &*a
// }


// fn main(){
//     let a = String::from("Shashashwat");
//     let c;
// {    
//     let b = String::from("Maahi");
//     c = tut2(&a, &b);
   
// }
//     println!("{}", c);
//     println!("dd {}", a);
//     }


// use std::fmt::Display;
// fn displayable<T: Display>(t: T) -> impl Display { t }
// fn main() {
//   let s = String::from("hello");
//   let mut s2 = displayable(s);
//   s2.push_str(" world");
//   println!("{s2}");
// }


// MUlti-threading 

// use std::thread;
// use std::time::Duration;
// use std::sync::mpsc;

// fn main (){
//     let (tx, rx) = mpsc::channel();

//     for i in 0..10 {
//         let prod = tx.clone();
//         thread::spawn(move || {
//             let mut sum: u64 =0;
//             for j in i*10000..(i+1)*10000 {
//                 sum = sum + j;
//             }
//             println!("thread {} sum {}", i, sum);
//             prod.send(sum).unwrap();
//         });
//     }

//     drop(tx);

//     let mut  ans: u64 = 0;
//     for rc in rx{
//         ans = ans + rc;

//     }
//     println!("sum is {}", ans);
// }
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("b.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn main(){
    let a = read_username_from_file();
    match a {
        Ok(data) => println!("Contents:\n{}", data),
        Err(e) => println!("Error: {}", e),
    }
}
