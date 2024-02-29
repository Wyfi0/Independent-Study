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
    
    // len returns the length (duh)
    println!("the length of ys is {}", ys.len()); // returns 500

}
