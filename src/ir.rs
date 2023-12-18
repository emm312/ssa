use std::{collections::HashSet, fmt::Display};

use crate::vcode::{InstrSelector, VCode, VCodeInstr, VCodeGenerator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub(crate) functions: Vec<Function>,
    pub name: String,
    pub(crate) analysis_stage: AnalysisStage,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AnalysisStage {
    LoweredToSSA,
    Optimised,
    Unanalyzed,
}

impl Module {
    pub fn new(name: &str, functions: Vec<Function>) -> Module {
        Module {
            functions,
            name: name.to_string(),
            analysis_stage: AnalysisStage::Unanalyzed,
        }
    }

    pub fn lower_to_vcode<I: VCodeInstr, S: InstrSelector<Instr = I> + Default>(&self) -> VCode<I> {
        let mut gen = VCodeGenerator::new();
        let mut selector = S::default();
        for func in self.functions.iter() {
            let f = gen.push_function(&func.name, func.linkage, func.args.len());
            gen.switch_to_func(f);
            for bb in func.blocks.iter() {
                let b = gen.push_block();
                gen.switch_to_block(b);
                for instr in bb.instructions.iter() {
                    selector.select(&mut gen, instr);
                }
                selector.select_terminator(&mut gen, &bb.terminator);
            }
        }
        gen.build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub(crate) ret_type: Type,
    pub(crate) args: Vec<(String, Type)>,
    pub(crate) blocks: Vec<BasicBlock>,
    pub(crate) linkage: Linkage,
    pub(crate) variables: Vec<Variable>,
    pub(crate) id: usize,
    pub(crate) values: Vec<Value>,
}

impl Function {
    pub(crate) fn new(
        name: &str,
        ret_type: Type,
        args: Vec<(String, Type)>,
        linkage: Linkage,
        variables: Vec<Variable>,
        id: usize,
    ) -> Self {
        Self {
            name: name.to_string(),
            ret_type,
            args,
            blocks: vec![],
            linkage,
            variables,
            id,
            values: vec![],
        }
    }

    pub(crate) fn push_block(&mut self, block: BasicBlock) {
        self.blocks.push(block);
    }

    pub(crate) fn push_value(&mut self, ty: Type) -> ValueId {
        let id = self.values.len();
        self.values.push(Value {
            ty,
            children: vec![],
            owner: BlockId(0),
        });
        ValueId(id)
    }

    pub(crate) fn replace_children_with(&mut self, original: ValueId, to_replace_to: ValueId) {
        for bb in self.blocks.iter_mut() {
            for instr in bb.instructions.iter_mut() {
                match &mut instr.operation {
                    Operation::BinOp(_, ref mut lhs, ref mut rhs) => {
                        if *lhs == original {
                            *lhs = to_replace_to;
                        }
                        if *rhs == original {
                            *rhs = to_replace_to;
                        }
                    }
                    Operation::Call(_, ref mut args) => {
                        for arg in args.iter_mut() {
                            if *arg == original {
                                *arg = to_replace_to;
                            }
                        }
                    }
                    Operation::StoreVar(.., ref mut val) => {
                        if *val == original {
                            *val = to_replace_to;
                        }
                    }
                    Operation::Phi(ref mut vals) => {
                        vals.iter_mut().for_each(|val| {
                            if *val == original {
                                *val = to_replace_to;
                            }
                        });
                    }
                    _ => (),
                }
            }
            match bb.terminator {
                Terminator::Return(ref mut val) => {
                    if *val == original {
                        *val = to_replace_to;
                    }
                }
                Terminator::Branch(ref mut val, ..) => {
                    if *val == original {
                        *val = to_replace_to;
                    }
                }
                _ => (),
            }
        }
        let mut c = self.values[original.0].children.clone();
        self.values[to_replace_to.0].children.append(&mut c);
        self.values[original.0].children.clear();
    }

    pub fn replace_instruction(&mut self, block: BlockId, instr: usize, new_instr: Instruction) {
        self.blocks[block.0].instructions[instr] = new_instr;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub(crate) name: String,
    pub(crate) ty: Type,
    pub(crate) bbs_assign_to: HashSet<BlockId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    pub(crate) ty: Type,
    pub(crate) children: Vec<ValueId>,
    pub(crate) owner: BlockId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Void,
    Integer(usize, bool),
    Pointer(Box<Type>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BasicBlock {
    pub(crate) name: String,
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) terminator: Terminator,
    pub(crate) preds: Vec<BlockId>,
    pub(crate) succs: Vec<BlockId>,
    pub(crate) id: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Terminator {
    Return(ValueId),
    Jump(BlockId),
    Branch(ValueId, BlockId, BlockId),
    NoTerm,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Linkage {
    Public,
    Private,
    External,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instruction {
    pub(crate) yielded: Option<ValueId>,
    pub(crate) operation: Operation,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operation {
    Integer(i64),
    BinOp(BinOp, ValueId, ValueId),
    Call(FunctionId, Vec<ValueId>),
    LoadVar(VariableId),
    StoreVar(VariableId, ValueId),
    Phi(Vec<ValueId>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "add"),
            BinOp::Sub => write!(f, "sub"),
            BinOp::Mul => write!(f, "mul"),
            BinOp::Div => write!(f, "div"),
            BinOp::Mod => write!(f, "mod"),
            BinOp::And => write!(f, "and"),
            BinOp::Or => write!(f, "or"),
            BinOp::Xor => write!(f, "xor"),
            BinOp::Shl => write!(f, "shl"),
            BinOp::Shr => write!(f, "shr"),
            BinOp::Eq => write!(f, "eq"),
            BinOp::Ne => write!(f, "ne"),
            BinOp::Lt => write!(f, "lt"),
            BinOp::Le => write!(f, "le"),
            BinOp::Gt => write!(f, "gt"),
            BinOp::Ge => write!(f, "ge"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub(crate) usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId(pub(crate) usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VariableId(pub(crate) usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub(crate) usize);

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "/* {} module {} */", self.analysis_stage, self.name)?;

        for func in &self.functions {
            write!(f, "{}", func)?;
        }

        Ok(())
    }
}

impl Display for AnalysisStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisStage::LoweredToSSA => {
                write!(f, "SSA form of")
            }
            AnalysisStage::Optimised => {
                write!(f, "Optimised form of")
            }
            AnalysisStage::Unanalyzed => {
                write!(f, "Unanalyzed")
            }
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "${}: fn {}({}) {} {{",
            self.id,
            self.name,
            self.args
                .iter()
                .map(|e| format!("{}: {}", e.0, e.1))
                .collect::<Vec<String>>()
                .join(", "),
            self.ret_type
        )?;

        for block in &self.blocks {
            write!(f, "{}", block)?;
        }

        write!(f, "}}")?;
        Ok(())
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "void")?,
            Type::Integer(size, signed) => {
                write!(f, "{}{}", if *signed { "s" } else { "u" }, size)?
            }
            Type::Pointer(ty) => write!(f, "{}*", ty)?,
        }
        Ok(())
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "${} (${}): ; preds = {}",
            self.name,
            self.id,
            self.preds
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        for instr in &self.instructions {
            writeln!(f, "    {}", instr)?;
        }
        writeln!(f, "    {}", self.terminator)?;
        Ok(())
    }
}

impl Display for Terminator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminator::Return(var) => write!(f, "ret {}", var)?,
            Terminator::Jump(block) => write!(f, "jmp ${}", block.0)?,
            Terminator::Branch(var, block1, block2) => {
                write!(f, "br {}, ${}, ${}", var, block1.0, block2.0)?
            }
            Terminator::NoTerm => write!(f, "noterm")?,
        }
        Ok(())
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(val) = self.yielded {
            write!(f, "{} = ", val)?;
        }
        write!(f, "{}", self.operation)
    }
}

impl Display for ValueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.0)
    }
}

impl Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::BinOp(op, lhs, rhs) => write!(f, "{} {} {}", op, lhs, rhs)?,
            Operation::Call(func, args) => write!(
                f,
                "call ${}({})",
                func.0,
                args.iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?,
            Operation::LoadVar(var) => write!(f, "load #{}", var.0)?,
            Operation::StoreVar(var, val) => write!(f, "store #{} {}", var.0, val)?,
            Operation::Integer(val) => write!(f, "{}", val)?,
            Operation::Phi(vals) => write!(
                f,
                "Φ {}",
                vals.iter()
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?,
        }
        Ok(())
    }
}
