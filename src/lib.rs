#![allow(dead_code)]

pub mod algos;
pub mod arch;
pub mod builder;
pub mod ir;
pub mod vcode;
pub mod regalloc;

#[cfg(test)]
mod tests {
    use crate::{
        algos::lower_to_ssa,
        builder::ModuleBuilder,
        ir::{BinOp, Terminator, Type, Linkage}, vcode::VCodeGenerator, arch::urcl,
    };

    #[test]
    fn fig_3dot1() {
        let mut builder = ModuleBuilder::new("fig3.1");

        let m_fn = builder.push_function("main", Type::Void, vec![], None);
        builder.switch_to_fn(m_fn);

        let entry = builder.push_block("entry");
        let bb_a = builder.push_block("a");
        let bb_b = builder.push_block("b");
        let bb_c = builder.push_block("c");
        let bb_d = builder.push_block("d");
        let bb_e = builder.push_block("e");

        builder.switch_to_block(entry);
        builder.set_terminator(Terminator::Jump(bb_a));

        builder.switch_to_block(bb_b);
        let x = builder.push_variable("x", Type::Integer(32, true)); // i32
        let y = builder.push_variable("y", Type::Integer(32, true)); // i32
        let val = builder.build_integer(0, Type::Integer(4, true));
        builder.build_store(x, val);
        builder.build_store(y, val);
        builder.set_terminator(Terminator::Jump(bb_d));

        builder.switch_to_block(bb_c);
        let tmp = builder.push_variable("tmp", Type::Integer(32, true));
        let ld_x = builder.build_load(x);
        let ld_y = builder.build_load(y);
        builder.build_store(tmp, ld_x);
        builder.build_store(x, ld_y);
        let ld_tmp = builder.build_load(tmp);
        builder.build_store(y, ld_tmp);
        let ld_x = builder.build_load(x);
        builder.set_terminator(Terminator::Branch(ld_x, bb_d, bb_e));

        builder.switch_to_block(bb_d);
        let ld_x = builder.build_load(x);
        let ld_y = builder.build_load(y);
        let val = builder.build_binop(BinOp::Add, ld_x, ld_y, Type::Integer(4, true));
        builder.build_store(x, val);
        let ld_x = builder.build_load(x);
        builder.set_terminator(Terminator::Branch(ld_x, bb_a, bb_e));

        builder.switch_to_block(bb_e);
        let ld_x = builder.build_load(x);
        builder.set_terminator(Terminator::Return(ld_x));

        builder.switch_to_block(bb_a);
        let ld_tmp = builder.build_load(tmp);
        builder.set_terminator(Terminator::Branch(ld_tmp, bb_b, bb_c));

        builder.print_module();

        let mut module = builder.build();
        lower_to_ssa::lower(&mut module);
        println!("{}", module);
    }

    #[test]
    fn test_var_renaming() {
        let mut builder = ModuleBuilder::new("test");
        let f = builder.push_function("main", Type::Void, vec![], None);
        builder.switch_to_fn(f);
        let entry = builder.push_block("entry");
        builder.switch_to_block(entry);
        let x = builder.push_variable("x", Type::Integer(32, true));
        let three = builder.build_integer(3, Type::Integer(32, true));
        builder.build_store(x, three);
        let ld_x = builder.build_load(x);
        builder.set_terminator(Terminator::Return(ld_x));
        builder.print_module();
        let mut module = builder.build();
        lower_to_ssa::lower(&mut module);
        println!("{}", module);
    }

    #[test]
    fn test_vcode() {
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

        let vcode = VCodeGenerator::<_, urcl::UrclSelector>::lower(&module);
        println!("{}", vcode);
    }
}
