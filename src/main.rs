// Shae Sullivan
// 20643731
// Programming Languages - Assignment
// 17/09/24


fn main() { //declare main method
    
    let string1 = String::from("im a string"); //declaring a mutable
    let string2 = &string1; //declaring a another string variable that uses &str to borrow string1's value without becoming the new owner

    //this allows string 1 to still be used as string 2 only borrowed string1
    println!("{}", string1); //print string1 to terminal
    println!("{}", string2); //print string2 to terminal

    fn hello_user(name: &str) { //declare function 

        println!("Hello, {}!", name);
    }

    hello_user("Shae");



}




