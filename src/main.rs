use ssa::{builder::ModuleBuilder, ir::{Type, Terminator, BinOp}};

fn main() {
    let mut builder = ModuleBuilder::new("test");
    let main_fn = builder.push_function("main", Type::Void, vec![], None);
    builder.switch_to_fn(main_fn);
    let bb = builder.push_block();
    builder.switch_to_block(bb);
    // generate code which needs phis
    let x = builder.push_variable("x", Type::Integer(32, true));
    let y = builder.push_variable("y", Type::Integer(32, true));
    let three = builder.build_integer(3, Type::Integer(32, true));
    builder.build_store(x, three);
    builder.build_store(y, three);
    let ld_x = builder.build_load(x);
    let edge = builder.push_block();
    let end = builder.push_block();
    builder.set_terminator(Terminator::Branch(ld_x, edge, end));

    builder.switch_to_block(edge);
    let ld_y = builder.build_load(y);
    let ld_x = builder.build_load(x);
    let val = builder.build_binop(BinOp::Add, ld_x, ld_y, Type::Integer(32, true));
    builder.build_store(x, val);
    builder.build_store(y, val);
    builder.set_terminator(Terminator::Jump(end));

    builder.switch_to_block(end);
    let ld_y = builder.build_load(y);
    builder.set_terminator(Terminator::Return(ld_y));
    builder.print_module();
    let mut m = builder.build();
    m.apply_mandatory_transforms();
    println!("{}", m)
}