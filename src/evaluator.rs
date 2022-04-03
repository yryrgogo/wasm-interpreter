use crate::module::number::{Number, NumberType};
use crate::module::value::Value;
use crate::structure::frame::Frame;
use crate::{module::Module, stack::Stack};

pub struct Evaluator {
    module: Module,
    stack: Stack,
}

impl Evaluator {
    fn call(&mut self, func_idx: usize) {
        let func = self.module.functions.get(func_idx).unwrap().clone();
        let mut args: Vec<Number> = vec![];

        for (i, _) in func.func_type.parameters.iter().enumerate() {
            let arg_type = func
                .func_type
                .parameters
                .get(func.func_type.parameters.len() - i - 1)
                .unwrap();
            let value = self.stack.pop_value();
            match value {
                Value::Int32(v) => {
                    if *arg_type == NumberType::Int32 {
                        args.push(Number::i32(Some(v)))
                    }
                    panic!("invalid popped value: {:?} {:?}", value, arg_type)
                }
                Value::Int64(v) => {
                    if *arg_type == NumberType::Int64 {
                        args.push(Number::i64(Some(v)))
                    }
                    panic!("invalid popped value: {:?} {:?}", value, arg_type)
                }
                Value::Float32(v) => {
                    if *arg_type == NumberType::Float32 {
                        args.push(Number::f32(Some(v)))
                    }
                    panic!("invalid popped value: {:?} {:?}", value, arg_type)
                }
                Value::Float64(v) => {
                    if *arg_type == NumberType::Float64 {
                        args.push(Number::f64(Some(v)))
                    }
                    panic!("invalid popped value: {:?} {:?}", value, arg_type)
                }
            };
        }

        self.stack.push_frame(Frame::new(func, args))
    }
}
