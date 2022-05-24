#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack:Vec<i32> = Vec::new();
    let mut res = None;

    for input in inputs{
        match input {
            CalculatorInput::Add => {
                if stack.len() < 2{
                    return None
                }
                let operand2 = stack.pop().expect("expected i32");
                let operand1 = stack.pop().expect("expected i32");

                let res = operand1 + operand2;

                stack.push(res);
            }
            CalculatorInput::Subtract => {
                if stack.len() < 2{
                    return None
                }
                let operand2 = stack.pop().expect("expected i32");
                let operand1 = stack.pop().expect("expected i32");

                let res = operand1 - operand2;

                stack.push(res);

            }
            CalculatorInput::Multiply => {
                if stack.len() < 2{
                    return None
                }
                let operand2 = stack.pop().unwrap_or(0);
                let operand1 = stack.pop().unwrap_or(0);

                let res = operand1 * operand2;

                stack.push(res);
            }
            CalculatorInput::Divide => {
                if stack.len() < 2{
                    return None
                }
                let operand2 = stack.pop().unwrap_or(0);
                let operand1 = stack.pop().unwrap_or(0);

                let res = operand1 / operand2;

                stack.push(res);
            }
            CalculatorInput::Value(v) => {stack.push(*v)}
        }
    }
    if stack.len() == 1{
        res = stack.pop();
    }

    res
}
