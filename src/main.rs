//tldr
//most stack based (smaller ones)  data are copied when assigned to another var or used in a control structure(like for
//loop) because copying new data into stack mem is fast.
//
//---i have no idea about large stack data-----
//
//heap based data are not copied but rather moved by default when they are assigned to another var or used in
//a control structure(like for loop) because the compiler calls the drop trait implementation
//automatically when the data goes out of scope 
//we can copy the heap data without moving it by calling the clone() method on it 
//(Note:operations on heap data is costly and should be dealt with caution)
//
//---this is for the data that doesnt get copied but instead gets moved-----
//using & 
//in a place like 
//let a=&something; means we are borrowing it i.e we are getting immutable reference to
//it not the data in the mem itself,i.e we get the ptr to that data immutable
//
//let a=&mut something; means we are getting a mutable reference of something,i.e borrow it mutable
//



#![allow(unused_variables)]
#![allow(dead_code)]
// use std::io;

// fn test(number:u32) {
//     println!("{number}");
// }

use std::vec;



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


    // let animals:[&str;3]=["cat","dog","rabbit"];
    // for a in animals {
    //     println!("The animal is {a}");
    // }
    // println!("trying to access 2nd time");
    // println!("using an array this time (i.e stack allocated memory)");
    //
    // for a in animals {
    //     println!("The animal is {a}");
    // }

    /*
     Notes:
        heap allocated data like String or vec! gets its ownership moved when using in a loop 
        stack allocated data like array doesnt 
    */


    //testing ownership moving

    // let inp_str=String::from("Hello,World!");
    // println!("Before moving");
    // println!("The value of inp_str is {inp_str}");
    // println!("Moving...");
    // // let sec_str=inp_str;    <= this moves the ownership
    // let sec_str=inp_str.clone(); //<= this copies (deep-copies) the data in a new heap memory and
    //                              //inits the sec_str
    // println!("The value of sec_str is {sec_str}");
    // println!("Trying to access inp_str");
    // println!("The value of inp_str is {inp_str}");





    //This would work because an array is alloc in the stack because whenever we use it in the for
    //loop,the array is copied,and used from there
    //This wont work on heap alloc data like vector because heap alloc data dont implement the
    //copy trait,i.e they are not copied into memory,they will be moved(called as the transfer of
    //ownership)(reason:refer top of the document)
    


    // let random_numbers:[i32;5]=[1,9,3,4,2];
    // for i in random_numbers{
    //     println!("The number is {i}");
    // }
    // println!("{}",random_numbers[3]);

    




    //Yo i figured it out
    //when we use just "for a in animals {...}" it means "for a in animals.into_iter(){...}"
    //into_iter() moves(not copy) it into a seperate memory for the loop
    //but when we use .iter() it returns &T for every item T in the list/vec 
    //it would mean something like "let a=&T;"
    //
    //
    // let animals=vec!["dog","cat","horse","boa","lizard wiz"];
    // for (i,a) in animals.iter().enumerate(){
    //     println!("The {i}th animal name is {a}");
    //     a="nigger";
    // }
    // println!("{}",animals[1]);

    //writing 
    //for &a in animals.iter(){...} is valid because each item in iter is &str and .iter() returns
    //a reference of it ,i.e &&str,when i use "for &a in animals.iter()" it basically means
    //let &a=&T; for every item T in animal 
    // the ampersand would cancel out and it would mean a=T
    // ^ this is called destructuring btw

    //thats the reason it wont work with enumerate,because it return an int 
    //and there is nothing to destructure 





    // // static STRING= String::from("hello");
    //
    // fn main() {
    //     let mut x = STRING; // convert &str → String
    //     x.push_str(" world");           // ✅ valid now
    //     println!("{x}");
    // }

    //Testing referencing and borrowing
    let animals=vec!["dog","cat","monkey","elephant"];

    for a in &animals.enumerate() {}


    
    






}
