//tldr
//most stack based (smaller ones)  data are copied when assigned to another var or used in a control structure(like for
//loop) because copying new data into stack mem is fast.
//
//^ just found out about this
//stack based data are always copied if they contain stack-types(i32,..) no matter how big they are
//if they contain a heap based type like String its moved no matter how big or small it is
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


// so its always better to borrow and use no matter what type heap or stack
// if i want to modify the original arr/vec i use iter_mut i.e borrow it mutably
// if i want to create a new list along with the original one,i should use .clone

//Some Great words by me
//“You can't make an immutable binding mutable, but you can move or borrow its value into something mutable.”
//i.e 
//let a=String::from("hello");
//let b=&mut a ; <- this is valid because we are mutating the value of a not a itself
//let b=mut a; <- invalid since we are trying to mutate the the var a itself
//
//Also !
//there can only be one reference which is muatble or there can be any number of immutable reference
//but not both,either 1 mut ref or n immut ref,you cannot have a immut ref along with another mut ref
//to tackle this you can create your desired ref in a seperate scope using a code  block

//constants
//constants are variables whose value gets directly substituted in the place they are used in the
//program automatically during the compilation time
//for example
//const MAX_SIZE:u8=20
//fn main() {   let a = MAX_SIZE  }  becomes let a=20 during compilation
//they cannot be modified and they are included directly in the binary
//i.e,no calculation is done during runtime to allocate and initialise the data in the const var
//They are immutable
//They allow scoping,i.e they cannot be used outside of the scope they are defined

#![allow(unused_variables)]
#![allow(dead_code)]
// use std::io;

// fn test(number:u32) {
//     println!("{number}");
// }

use std::{io, vec};



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
    // let animals=vec!["dog","cat","monkey","elephant"];
    //
    // for a in animals.iter().enumerate() {}


    //testing enums

    // enum Colors {
    //     White,
    //     Yellow,
    //     Red,
    //     Orange,
    //     Blue,
    //     Green
    // }
    //
    // let c=Colors::Blue;
    //
    // match c {
    //     Colors::White => println!("valid color in a rubik's cube"),
    //     Colors::Yellow => println!("valid color in a rubik's cube"),
    //     Colors::Red => println!("valid color in a rubik's cube"),
    //     Colors::Orange => println!("valid color in a rubik's cube"),
    //     Colors::Blue => println!("valid color in a rubik's cube"),
    //     Colors::Green => println!("valid color in a rubik's cube"),
    //     _ => println!("Not a valid rubik;s cube color");
    // }

    //testing tuples
    // let tup=(1,2,3,false,6.9); 
    // println!("{}",tup.2+tup.1);


    // the below wont work because tuple doesnt impement .iter() because it's
    // heterogenous(collection of diff types)
    // for t in tup.iter() {
    //     println!("{t}");
    // }

    // let tup2=(1,true,String::from("Hello"),(2,3,String::from(",World!")));
    //^ A tuple can be nested and can even have heap based data in it like strings.
    //^this decides if the tuple is stored in the heap or stack

    //we can acces elements of a tuple using . operator like tup.0 or tup.1 ,i.e tup.index
    //to access element of a nested tup ,use (tup.idx1).idx2

    //testing structs
    // struct Person {
    //     name:String,
    //     age:u8,
    //     gender:char
    // }
    //
    // let person1=Person{name:String::from("sugan"),age:19,gender:'m'};
    // println!("My name is {} and i am {} years old",person1.name,person1.age);


    //testing tuple structs
    //  struct Person(String,u8,char);
    //
    // let p1=Person(String::from("sugan"),19,'m');
    // println!("My name is {} and i am {} years old",p1.0,p1.1);
    // testing for mutability
    // struct Person(String,u8,char);
    //use mut here because we need to change the field of the struct
    // let mut p1=Person(String::from("sugan"),19,'m');
    // p1.0=String::from("Suganraj");
    // println!("My name is {} and i am {} years old",p1.0,p1.1);




    
}
