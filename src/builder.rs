use crate::ir::{
    BasicBlock, BinOp, BlockId, Function, FunctionId, Instruction, Linkage, Module, Operation,
    Terminator, Type, ValueId, Variable, VariableId,
};

pub struct ModuleBuilder {
    module: Module,
    current_func: Option<FunctionId>,
    current_block: Option<BlockId>,
}

impl ModuleBuilder {
    pub fn new(name: &str) -> ModuleBuilder {
        ModuleBuilder {
            module: Module::new(name, Vec::new()),
            current_block: None,
            current_func: None,
        }
    }

    pub fn print_module(&self) {
        println!("{}", self.module);
    }

    pub fn push_function(
        &mut self,
        name: &str,
        ret_type: Type,
        args: Vec<(String, Type)>,
        linkage: Option<Linkage>,
    ) -> FunctionId {
        self.module.functions.push(Function::new(
            name,
            ret_type,
            args,
            linkage.unwrap_or(Linkage::Private),
            vec![],
            self.module.functions.len(),
        ));
        FunctionId(self.module.functions.len() - 1)
    }

    pub fn push_block(&mut self, name: &str) -> BlockId {
        let id = self
            .module
            .functions
            .get(self.current_func.unwrap().0)
            .unwrap()
            .blocks
            .len();
        self.module.functions[self.current_func.as_ref().unwrap().0]
            .blocks
            .push(BasicBlock {
                name: name.to_string(),
                instructions: vec![],
                terminator: Terminator::NoTerm,
                id,
                preds: Vec::new(),
                succs: Vec::new(),
            });
        BlockId(id)
    }

    pub fn switch_to_fn(&mut self, id: FunctionId) {
        self.current_func = Some(id);
    }

    pub fn switch_to_block(&mut self, id: BlockId) {
        self.current_block = Some(id);
    }

    pub fn build_binop(&mut self, op: BinOp, lhs: ValueId, rhs: ValueId) -> ValueId {
        let val = self.push_value();
        let block = self.get_block_mut(self.current_block.unwrap());
        block.instructions.push(Instruction {
            yielded: Some(val),
            operation: Operation::BinOp(op, lhs, rhs),
        });
        val
    }

    fn get_func(&self, id: FunctionId) -> &Function {
        &self.module.functions[id.0]
    }

    fn get_func_mut(&mut self, id: FunctionId) -> &mut Function {
        &mut self.module.functions[id.0]
    }

    fn get_block(&self, id: BlockId) -> &BasicBlock {
        &self.get_func(self.current_func.unwrap()).blocks[id.0]
    }

    fn get_block_mut(&mut self, id: BlockId) -> &mut BasicBlock {
        &mut self.get_func_mut(self.current_func.unwrap()).blocks[id.0]
    }

    pub fn push_variable(&mut self, name: &str, ty: Type) -> VariableId {
        let func = self.get_func_mut(self.current_func.unwrap());
        func.variables.push(Variable {
            name: name.to_string(),
            ty,
        });
        VariableId(func.variables.len() - 1)
    }

    pub fn build_integer(&mut self, value: i64) -> ValueId {
        let val = self.push_value();
        let block = self.get_block_mut(self.current_block.unwrap());
        block.instructions.push(Instruction {
            yielded: Some(val),
            operation: Operation::Integer(value),
        });
        val
    }

    pub fn build_store(&mut self, var: VariableId, value: ValueId) {
        let block = self.get_block_mut(self.current_block.unwrap());
        block.instructions.push(Instruction {
            yielded: None,
            operation: Operation::StoreVar(var, value),
        });
    }

    pub fn build_load(&mut self, var: VariableId) -> ValueId {
        let val = self.push_value();
        let block = self.get_block_mut(self.current_block.unwrap());
        block.instructions.push(Instruction {
            yielded: Some(val),
            operation: Operation::LoadVar(var),
        });
        val
    }

    pub fn set_terminator(&mut self, terminator: Terminator) {
        let cur_blk = self.current_block.unwrap();
        match terminator {
            Terminator::Return(_) => {}
            Terminator::Jump(loc) => {
                self.get_block_mut(loc).preds.push(cur_blk);
                self.get_block_mut(self.current_block.unwrap())
                    .succs
                    .push(loc);
            }
            Terminator::Branch(_, loc1, loc2) => {
                self.get_block_mut(loc1).preds.push(cur_blk);
                self.get_block_mut(loc2).preds.push(cur_blk);
                self.get_block_mut(self.current_block.unwrap())
                    .succs
                    .push(loc1);
                self.get_block_mut(self.current_block.unwrap())
                    .succs
                    .push(loc2);
            }
            _ => panic!("tried to set terminator to noterm"),
        }
        self.get_block_mut(self.current_block.unwrap()).terminator = terminator;
    }

    // internal function to init values
    #[inline]
    fn push_value(&mut self) -> ValueId {
        self.get_func_mut(self.current_func.unwrap()).value_counter += 1;
        ValueId(self.get_func(self.current_func.unwrap()).value_counter - 1)
    }
}
