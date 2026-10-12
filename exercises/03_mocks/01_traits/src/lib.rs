//! Refactor the `square` function to ask for a type that implements the `Logger` trait rather than the concrete
//! `PrintlnLogger` type.\
//! Then pass a `TestLogger` to `square` in the test. `TestLogger` should implement `Logger` and do nothing
//! when `log` is called.

pub fn square<L>(x: i32, logger: L) -> i32
where
    L: Logger
{
    let y = x * x;
    logger.log(&format!("{}^2 == {}", x, y));
    y
}

trait Logger {
    fn log(&self, msg: &str);
}

pub struct TestLogger;
pub struct PrintlnLogger;

impl Logger for TestLogger {
    fn log(&self, msg: &str) {}
}

impl Logger for  PrintlnLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::{square, TestLogger};
    use googletest::assert_that;
    use googletest::matchers::eq;

    #[test]
    fn square_works() {
        assert_eq!(square(2, TestLogger), 4);
    }
}
