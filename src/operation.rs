pub enum Operations {
    Add(i32, i32),
    Subtraction(i32, i32),
    Multiplication(i32, i32),
    Division(i32, i32),
}

pub fn calculate(op: Operations) -> Result<i32, &'static str> {
    match op {
        Operations::Add(x, y) => Ok(x + y),
        Operations::Subtraction(x, y) => Ok(x - y),
        Operations::Multiplication(x, y) => Ok(x * y),
        Operations::Division(x, y) => {
            if y > 0 {
                return Ok(x / y);
            }
            return Err("Can't divide by zero!");
        }
    }
}
