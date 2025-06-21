#![allow(unused_variables)]
#![allow(dead_code)]

use std::io;
fn main() {
    // let s:&str="Hello,World";
    // println!("the value of s is {s}");
    //
    // let mut arr:[u32;8]=[0;8];
    //
    // for i in 0..=7 {
    //     arr[i]=i as u32;
    // }
    //
    // for (i,&value) in arr.iter().enumerate(){
    //     println!("The value in the {i}-th index is {value}");
    // }

    // let s1=String::from("Hello,World!");
    // println!("the value of s1 is {s1}");
    // let s2=s1.clone();
    // println!("copied s1 to s2");
    // println!("Moved s1's ownership to s2");
    // println!("the value of s2 is {s2}");
    // println!("Attempting to access s1 : {s1}");

    // fn chigga(name: &str) -> String {
    //     return String::new();
    // }

    //testing with refernce
    // let s1=String::from("Open your ass Nisha");
    // let s2=&s1;
    // println!("The value of s1 is {s1} and the value of s2 is {s2} and the value of address of s1 is {} and s2 is {}",&s1,&s2);
    // println!("Trying to access again");
    // println!("The value of s1 is {s1} and the value of s2 is {s2} and the value of address of s1 is {} and s2 is {}",&s1,&s2);

    // fn fibonnaci(lvl: i32) -> bool {
    //     println!("Hello from fib fn !");
    //     true
    // }
    //testing with integers
    // let  mut x:[u32;5]=[0;5];
    // for i in 0..5 {
    //     x[i]=i as u32;
    // }
    // println!("successfully initialised x");



    //testing string slicing


    // let mut inp_str=String::new();
    // println!("Input a string:");
    // io::stdin().read_line(&mut inp_str).expect("invalid input");
    // fn first_word(sent:&str) -> &str {
    //     let mut occurence:i32=0;
    //     for i in sent.split_terminator(" "){
    //         if i!=" "{
    //             i
    //         }
    //     }
    //     sent
    // }
    //


                                                             //trailing spaces from a line
    // let first_w=first_word(&mut inp_str);
    // println!("{first_w}");
    //

    //slicing the first word from the sentence
    // fn first_word(sentence:&String) -> &str {
    //     let mut trimmed_str:&str=sentence.trim();
    //     for i in 0..trimmed_str.len(){
    //         if trimmed_str[i]==" " {
    //             return &trimmed_str[0..i];
    //         }
    //     }
    //     trimmed_str
    // }


    // fn first_word(sentence:&String)->&str{
    //     let s=sentence.as_bytes();
    //     for (i,&item) in s.iter().enumerate() {
    //         if item==b' ' {
    //             return &sentence[0..i];
    //         }
    //     }
    //     &sentence[..]
    // }
    //








}
