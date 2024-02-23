fn main() {
    println!("Hello, world!");
    let x = 10;
    let y = &x;
    let z =&y;
    println!("{}", y);
    println!("{z}");

    let mut x =10;
    {
        let z = &mut x;
        *z +=1;
        println!("The value of f{}", &z);
    }
    println!("The value of x is: {x}");
}
