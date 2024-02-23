fn main() {
    println!("Hello, world!");
    first();
}
fn first(){
    let x =90;
    // hence x is the oewner of 90
    println!("{x}");
    let y =x;//here the value of x is transferred to y but x is the acutal owner of 90
    //hence it saves the menory
    let z=90; //this is wastage of memory
    let a = String::from("Pankaj Rathor");
    let b = a.clone();
    println!("{}-------->{}",a, b);
}