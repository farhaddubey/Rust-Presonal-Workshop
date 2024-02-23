fn main() {
    println!("If Else conditions ---");
    first();
    second();
    third();
}
fn first(){
    let number=4;
    if number<5{
        println!("The no .is smaller");
    }else{
        let abc= 54+23;
        println!("{abc}");
        println!("False");
    }
}
fn second(){
    let no = 3;
    if no%4==0{
        println!("No is divisble by 4");
    }else if no%6==0{
        println!("No is divisble by 6");
    }else if no%7==0{
        println!("No is divisble by 8");
    }else{
        println!("In else block");
    }
}
 
fn third(){
    let condition = true;
    let number = if condition {5} else {0};
    println!("The value of the number is: {}", number);
} 