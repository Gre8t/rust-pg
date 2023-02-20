fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        //_mutable_integer = 50;
       
    }

    // NB: Rust only allows 1 borrower of a variable per scope. Unlike Go
    _mutable_integer = 3;
    println!("mutable integer is: {}", _mutable_integer);
    // another method of displaying variable freezing
    let mut new_mutable_integer = 10;

     {
        let new_immutable_integer = &new_mutable_integer;
        
        new_mutable_integer = 4;
        // this should not go through an error because 4 _new_mutable_integer
        // has been frozen
        println!("this is the new integer: {}", new_immutable_integer);
        //although there is a little nuance in the above code
     }

    
}