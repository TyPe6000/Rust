fn main()
{
    let mut s = String::from("hello");
    
    s.push_str(", world!"); // error: cannot borrow `s` as mutable, as it is not declared as mutable

    println!("{}", s);
}
