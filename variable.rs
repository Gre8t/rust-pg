fn main(){
    let long_lived_binding = 1;
    {
        let long_lived_binding = 2;
        
        println!("innner long: {}", long_lived_binding);
    }
    
    println!("outer long: {}", long_lived_binding);
    //outer shadowing of the existing variable
    let long_lived_binding = 4;
    println!("outer shadow: {}", long_lived_binding)

}