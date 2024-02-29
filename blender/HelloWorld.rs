fn main() {
    // this is a comment
    /*
      pretty simple 
    */
    // Let it beeeee!
    let x = 5;

    // {} is replaced by arguments seperated by a comma
    println!("x equals {}", x);

    // you can also specify the arguments by the index in the following arguments
    println!("my name is {0}. your name is {1}.", "me", "you");
    
    // you can also name the arguments
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "The quick brown fox",
             verb="jumps over");

    // you can also use different foramtting tags
    println!("Base 10: {}", 69420); // 69420
    println!("Base 2 (binary): {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal): {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C
   
    // you can rught-justify text with :
    println!("{number:>5}", number=1);

    // you can pad nums with zeroes
    println!("{number:0<5}", number=1); // 00001
    println!("{number:0>5}", number=1); // 10000
    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    let pi = 3.141592;
    println!("pi to 3 places is: {pi:.3}");
    println!("pi to 6 places is: {pi:.6}");

}
