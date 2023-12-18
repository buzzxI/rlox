mod scanner;
mod compiler;
mod interpreter;

pub enum InterpreterResult {
    InterpreterOk,
    InterpreterCompileError,
    // InterpreterRuntimeError,
    // InterpreterDebug,
}

pub fn interpret(source: &str) -> InterpreterResult  {
    let function = match compiler::compile(source) {
        Some(function) => function,
        None => return InterpreterResult::InterpreterCompileError,
    };
    function.print();
    InterpreterResult::InterpreterOk
} 
