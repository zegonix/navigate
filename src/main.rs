use std::env;


fn main()
{
    let mut args : Vec<String> = env::args().collect();
    let pid  : i32         = args[1].parse().unwrap();

    // remove arguments which don't require parsing
    args.remove(0); // remove command name
    args.remove(0); // remove pid

    for arg in args
    {
        if !(arg.clone().chars().nth(0).unwrap() == '-')
        {
            println!("{}", arg)
        }
    }    
}
