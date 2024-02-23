fn main() {
    println!("Hello, world!");
    first_fn();
    second_fn(200);
    third_fn(200, 'h');
    // help: if you meant to write a `char` literal, use single quotes
    ex();
    let xy = return_value();    
    println!("The value of xy is: {xy}")

}
fn first_fn(){
    println!("Hey there I'm the 2nd rust function dude that u rr writting");
}
fn second_fn(x: i32){
    println!("The value of x is: - {}", x);
}
fn third_fn(x:i32, y:char){
    println!("The value of x is: {x} & th evalue of y is :{y}");
}
//expressioons, where function doesnt' providde value to user directly
fn ex(){
    let y={
        let x=10;
        x+1
    };
    println!("Value of y is: {}",y);
}
//return value from function
fn return_value()->i32{
    89+90
}
