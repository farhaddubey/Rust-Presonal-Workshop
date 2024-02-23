fn main() {
    println!("Hello, world!");
    //Scalar type
    //Integer String Boolean Floarting Char
    //length(8bit, 16, 32, 64, 128)


    let tup = (32, 45, 43);
    println!("{:?}", tup);
    println!("{}", tup.1);

    let mut tup1: (i32, u8, f64) = (32, 43, 56.9);
    println!("{:?}", tup);
    println!("{}", tup.1);

    // TO chagen value of tuple we've to make mutable tupple by using : let mut tup1 
    tup1.0=100;
    println!("{:?}", tup1);
    println!("{}", tup1.0);
}
