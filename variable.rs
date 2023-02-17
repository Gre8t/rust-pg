fn main(){
    //declare first
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }    
    println!("a binding: {}", a_binding);

    let another_binding;
// use a declared variable that is not yet initiallized
    //println!("another binding:{}", another_binding)
    another_binding = 1;
// if you print here it should work fine without any errors
    println!("another binding: {}", another_binding);
    
// This is a key difference with Go. Rust doesn't allow uninited 
// variables to be used but you can do this in Go. Although Go
// automatically set the value to nil, Rust doesn't allow it.
}