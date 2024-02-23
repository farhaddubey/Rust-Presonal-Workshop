fn main() {
    println!("Hello, world!");
    first();
    second();
    third();
}
fn first(){
    let mut x=0;

    loop{
        x+=1;
        println!("x is {x}");
        if x==5{
            println!("We reached upto destiny, hence coming to finish");
            break;
        }
    }
}
fn second(){
    let mut number =0;
    while number !=0{
        println!("{number}");

    }
    println!("Hello ");
}
fn third(){
    let a =[10, 20, 30, 40, 50];
    let mut index =0;
    // while index<6{
    //     println!("The value is: {}", a[index]);
    // }
    // index +=1;
    for x in 0..10{
        if x==6{
            continue;
        }
        println!("x is {x}");

    }
}
