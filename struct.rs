struct User{
    name: String,
    company: String,
    age: u32
}
struct Home{
    house_no: i32,
    house_name: String,
    you_lived:bool
}
fn main() {
    let u1 = User{
        name: String::from("Farhad Dubey"),
        company: String::from("TDIL Consultancy Services, TOP World Company"),
        age: 100
    };
    let um1 = Home{
        house_no: 455,
        house_name: String::from("Shanti Niwas"),
        you_lived: true
    };
    println!("My house no is{}, house name{}, I lived is, {}", um1.house_no, um1.house_name, um1.you_lived);
    println!("The name in struct is: {}, company is {}, age is :{}", u1.name, u1.company, u1.age);
    //This declaration from outside support s for all
    //We can change the u1 data as that's not declares as mutable
    // To change make it mutable
    let mut u2 = User{
        name: String::from("Farhad Dubey"),
        company: String::from("TDIL Consultancy Services, TOP World Company"),
        age: 100
    };
    u2.name= String::from("Great Farhad Dubey");
    println!("The name in struct is: {}, company is {}, age is :{}", u2.name, u1.company, u1.age);
}
