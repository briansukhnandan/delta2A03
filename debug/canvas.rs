fn main() {

    //let mut i = 0;
    // for mut i in 0..10 {
    //     if i == 1 {
    //         i = i + 2;
    //     }
    //     println!("{}", i);
    // }

    let a: [u8; 0xFF+1] = [45; 0xFF+1];

    println!("{}", a[0xFF])
    
}