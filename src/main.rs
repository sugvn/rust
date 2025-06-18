#![allow(unused_variables)]


fn main() {
    let s:&str="Hello,World";
    println!("the value of s is {s}");

    let mut arr:[u32;8]=[0;8];
    for i in 0..=7 {
        arr[i]=i as u32;
    }


    for (i,&value) in arr.iter().enumerate(){
        println!("The value in the {i}-th index is {value}");
    }

}
