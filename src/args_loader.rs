use std::env::Args;
use std::collections::BinaryHeap;

pub fn get_args(args: Args) -> Vec<String>   {

    let _args : Vec<String> = args.skip(1).collect();
    let len : usize         = _args.len();

    assert_ne!(len as i32, 0, "Arguments not found.");


    return _args
}

pub fn load() {}

#[test]
pub fn get_args_test()    {

    let env_test_arg              = &["A","B","C","D"];
    let mut _args     : Vec<String>           = Vec::new();

    for i in env_test_arg.iter() {
        _args.push(i.parse().unwrap());
    }

    let len           : usize                 = _args.len();

    assert_ne!(len as i32, 0);

    println!("{:#?}",_args)
}