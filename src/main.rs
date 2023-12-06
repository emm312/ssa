use ssa::{
    builder::ModuleBuilder,
    ir::{BinOp, Function, Linkage, Module, Terminator, Type},
};

fn main() {
    let mut builder = ModuleBuilder::new("main");
    let main_fn = builder.push_function("main", Type::Void, vec![], None);
    builder.switch_to_fn(main_fn);
    let entry_block = builder.push_block("entry");
    builder.switch_to_block(entry_block);
    let a = builder.build_integer(54);
    let b = builder.build_integer(5);
    let op = builder.build_binop(BinOp::Add, a, b);
    let a_bb = builder.push_block("a");
    let b_bb = builder.push_block("b");
    builder.set_terminator(Terminator::Branch(b, a_bb, b_bb));

    builder.print_module();
}
