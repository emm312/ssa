use ssa::{algos::lower_to_ssa, ir::{Terminator, BinOp, Type}, arch::urcl::UrclSelector, builder::ModuleBuilder};

fn main() {
    let mut builder = ModuleBuilder::new("test");
    let func = builder.push_function("main", Type::Integer(32, true), vec![], None);
    builder.switch_to_fn(func);

    let entry = builder.push_block("entry");
    builder.switch_to_block(entry);

    let x = builder.push_variable("x", Type::Integer(32, true));
    let y = builder.push_variable("y", Type::Integer(32, true));
    let three = builder.build_integer(3, Type::Integer(32, true));
    builder.build_store(x, three);
    builder.build_store(y, three);
    let ld_x = builder.build_load(x);
    let ld_y = builder.build_load(y);
    let val = builder.build_binop(BinOp::Add, ld_x, ld_y, Type::Integer(32, true));

    builder.set_terminator(Terminator::Return(val));
    let mut module = builder.build();
    lower_to_ssa::lower(&mut module);
    println!("{}", module);
    let vcode = module.lower_to_vcode::<_, UrclSelector>();
    println!("{}", vcode);
}