use crate::args_loader::AnyStruct;

mod args_loader;

mod struct_data_lib;

fn main() {

    let args = args_loader::get_args(std::env::args());
    let a : AnyStruct = AnyStruct{x: 1.0, y: 2.0};
    AnyStruct::foo(args);

    struct_data_lib::qdb_structs::QBinaryTree
}