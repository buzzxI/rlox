enum ObjectType {
    LoxString,
    LoxFunction,
    LoxNative,
    LoxClosure,
    LoxUpvalue,
    LoxClass,
    LoxInstance,
    LoxMethod,
}

pub struct FunctionObj {
    obj_type: ObjectType,
    arity: u8,
    name: String,
    // chunk: Chunk,
    upvalue_count: u8,
}

impl FunctionObj {
    pub fn new(name: String) -> Self {
        Self {
            obj_type: ObjectType::LoxFunction,
            arity: 0,
            name,
            // chunk: Chunk::new(),
            upvalue_count: 0,
        }
    }

    pub fn print(&self) {
        println!("<rlox function: {}>", self.name);
    }
}