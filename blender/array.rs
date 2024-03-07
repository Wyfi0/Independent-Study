// 'Import' mem from std
use std::mem;

// Borrow a slice (a two part array not knowing the length at compile)
// explicitly take a 32 bit integer and name it slice?
fn analyze_slice(slice: &[i32]) {
    // print the first element like an array
    println!("First element of the slice is {}", slice[0]);
    // prin the length of the slice
    println!("There are {} elements in this slice.", slice.len());
}

fn main() {
    // Fixed size array (dont actually need type signature)
    let xs: [i32; 5] = [1,2,3,4,5]; // type and length, and then the elements

    // Set all elements to same value (in this case 1)
    let ys: [i32; 500] = [1; 500];
    /*
    // uncomment to print all of ys
    for x in ys {
        print!("{}, ", x);
    }
    */
    
    // .len returns the length (duh)
    println!("the length of ys is {}", ys.len()); // returns 500

    // Use mem::size_of_val to get byte size of array
    println!("Array xs takes up {} bytes", mem::size_of_val(&xs));
    println!("Array ys takes up {} bytes", mem::size_of_val(&ys));

    // Slices can point to section of array
    // its like [starting_index..ending_index]
    // starting is the first part
    // ending_index is ONE MORE than the last position of the slice (wtf)
    println!("Borrow section of array ys as slice");
    analyze_slice(&ys[0 .. 8]); // [start .. one more than finish]
    // arrays accessed using '.get', returns an 'Option'.
    // This can be matched or used with '.expect()' if you want the program to exit 
    // nicely instead of continue
    for i in 0..xs.len() {
        match xs.get(i) {
            Some(xval)  => println!("{}: {}", i, xval),
            None => println!("Slow down, {} is too far!", i),
        }
    }

    // Out of bound indexing on array causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);
}
