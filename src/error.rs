use std::{fmt, io};

use crate::{operator::Operator, token::PartialToken, value::value_type::ValueType, value::Value};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// A row was inserted with a wrong amount of arguments.
    WrongColumnAmount {
        /// The expected amount of arguments.
        expected: usize,
        /// The actual amount of arguments.
        actual: usize,
    },

    /// An operator was called with a wrong amount of arguments.
    WrongOperatorArgumentAmount {
        /// The expected amount of arguments.
        expected: usize,
        /// The actual amount of arguments.
        actual: usize,
    },

    /// A function was called with a wrong amount of arguments.
    WrongFunctionArgumentAmount {
        /// The expected amount of arguments.
        expected: usize,
        /// The actual amount of arguments.
        actual: usize,
    },

    /// A string value was expected.
    ExpectedString {
        /// The actual value.
        actual: Value,
    },

    /// An integer value was expected.
    ExpectedInt {
        /// The actual value.
        actual: Value,
    },

    /// A float value was expected.
    ExpectedFloat {
        /// The actual value.
        actual: Value,
    },

    /// A numeric value was expected.
    /// Numeric values are the variants `Value::Int` and `Value::Float`.
    ExpectedNumber {
        /// The actual value.
        actual: Value,
    },

    /// A numeric or string value was expected.
    /// Numeric values are the variants `Value::Int` and `Value::Float`.
    ExpectedNumberOrString {
        /// The actual value.
        actual: Value,
    },

    /// A boolean value was expected.
    ExpectedBoolean {
        /// The actual value.
        actual: Value,
    },

    /// A tuple value was expected.
    ExpectedTuple {
        /// The actual value.
        actual: Value,
    },

    /// A tuple value of a certain length was expected.
    ExpectedFixedLenTuple {
        /// The expected len
        expected_len: usize,
        /// The actual value.
        actual: Value,
    },

    /// An empty value was expected.
    ExpectedEmpty {
        /// The actual value.
        actual: Value,
    },

    /// A map value was expected.
    ExpectedMap {
        /// The actual value.
        actual: Value,
    },

    /// A map value was expected.
    ExpectedTable {
        /// The actual value.
        actual: Value,
    },

    /// A map value was expected.
    ExpectedFunction {
        /// The actual value.
        actual: Value,
    },

    /// Tried to append a child to a leaf node.
    /// Leaf nodes cannot have children.
    AppendedToLeafNode,

    /// Tried to append a child to a node such that the precedence of the child is not higher.
    /// This error should never occur.
    /// If it does, please file a bug report.
    PrecedenceViolation,

    /// A `VariableIdentifier` operation did not find its value in the context.
    VariableIdentifierNotFound(String),

    /// A `FunctionIdentifier` operation did not find its value in the context.
    FunctionIdentifierNotFound(String),

    /// A value has the wrong type.
    /// Only use this if there is no other error that describes the expected and provided types in more detail.
    TypeError {
        /// The expected types.
        expected: Vec<ValueType>,
        /// The actual value.
        actual: Value,
    },

    /// An operator is used with a wrong combination of types.
    WrongTypeCombination {
        /// The operator that whose evaluation caused the error.
        operator: Operator,
        /// The types that were used in the operator causing it to fail.
        actual: Vec<ValueType>,
    },

    /// An opening brace without a matching closing brace was found.
    UnmatchedLBrace,

    /// A closing brace without a matching opening brace was found.
    UnmatchedRBrace,

    /// Left of an opening brace or right of a closing brace is a token that does not expect the brace next to it.
    /// For example, writing `4(5)` would yield this error, as the `4` does not have any operands.
    MissingOperatorOutsideOfBrace,

    /// A `PartialToken` is unmatched, such that it cannot be combined into a full `Token`.
    /// This happens if for example a single `=` is found, surrounded by whitespace.
    /// It is not a token, but it is part of the string representation of some tokens.
    UnmatchedPartialToken {
        /// The unmatched partial token.
        first: PartialToken,
        /// The token that follows the unmatched partial token and that cannot be matched to the partial token, or `None`, if `first` is the last partial token in the stream.
        second: Option<PartialToken>,
    },

    /// An addition operation performed by Rust failed.
    AdditionError {
        /// The first argument of the addition.
        augend: Value,
        /// The second argument of the addition.
        addend: Value,
    },

    /// A subtraction operation performed by Rust failed.
    SubtractionError {
        /// The first argument of the subtraction.
        minuend: Value,
        /// The second argument of the subtraction.
        subtrahend: Value,
    },

    /// A negation operation performed by Rust failed.
    NegationError {
        /// The argument of the negation.
        argument: Value,
    },

    /// A multiplication operation performed by Rust failed.
    MultiplicationError {
        /// The first argument of the multiplication.
        multiplicand: Value,
        /// The second argument of the multiplication.
        multiplier: Value,
    },

    /// A division operation performed by Rust failed.
    DivisionError {
        /// The first argument of the division.
        dividend: Value,
        /// The second argument of the division.
        divisor: Value,
    },

    /// A modulation operation performed by Rust failed.
    ModulationError {
        /// The first argument of the modulation.
        dividend: Value,
        /// The second argument of the modulation.
        divisor: Value,
    },

    /// A regular expression could not be parsed
    InvalidRegex {
        /// The invalid regular expression
        regex: String,
        /// Failure message from the regex engine
        message: String,
    },

    /// A modification was attempted on a `Context` that does not allow modifications.
    ContextNotMutable,

    /// An escape sequence within a string literal is illegal.
    IllegalEscapeSequence(String),

    /// This context does not allow enabling builtin functions.
    BuiltinFunctionsCannotBeEnabled,

    /// This context does not allow disabling builtin functions.
    BuiltinFunctionsCannotBeDisabled,

    /// The function failed due to an external error.
    FunctionFailure(String),

    /// A custom error explained by its message.
    CustomMessage(String),
}

impl From<csv::Error> for Error {
    fn from(value: csv::Error) -> Self {
        Error::FunctionFailure(value.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::FunctionFailure(value.to_string())
    }
}

impl From<git2::Error> for Error {
    fn from(value: git2::Error) -> Self {
        Error::FunctionFailure(value.to_string())
    }
}

impl From<trash::Error> for Error {
    fn from(value: trash::Error) -> Self {
        Error::FunctionFailure(value.to_string())
    }
}

impl From<sys_info::Error> for Error {
    fn from(value: sys_info::Error) -> Self {
        Error::FunctionFailure(value.to_string())
    }
}

impl Error {
    pub(crate) fn wrong_operator_argument_amount(actual: usize, expected: usize) -> Self {
        Error::WrongOperatorArgumentAmount { actual, expected }
    }

    pub(crate) fn wrong_function_argument_amount(actual: usize, expected: usize) -> Self {
        Error::WrongFunctionArgumentAmount { actual, expected }
    }

    /// Constructs `EvalexprError::TypeError{actual, expected}`.
    pub fn type_error(actual: Value, expected: Vec<ValueType>) -> Self {
        Error::TypeError { actual, expected }
    }

    /// Constructs `EvalexprError::WrongTypeCombination{operator, actual}`.
    pub fn wrong_type_combination(operator: Operator, actual: Vec<ValueType>) -> Self {
        Error::WrongTypeCombination { operator, actual }
    }

    /// Constructs `EvalexprError::ExpectedString{actual}`.
    pub fn expected_string(actual: Value) -> Self {
        Error::ExpectedString { actual }
    }

    /// Constructs `EvalexprError::ExpectedInt{actual}`.
    pub fn expected_int(actual: Value) -> Self {
        Error::ExpectedInt { actual }
    }

    /// Constructs `EvalexprError::ExpectedFloat{actual}`.
    pub fn expected_float(actual: Value) -> Self {
        Error::ExpectedFloat { actual }
    }

    /// Constructs `EvalexprError::ExpectedNumber{actual}`.
    pub fn expected_number(actual: Value) -> Self {
        Error::ExpectedNumber { actual }
    }

    /// Constructs `EvalexprError::ExpectedNumberOrString{actual}`.
    pub fn expected_number_or_string(actual: Value) -> Self {
        Error::ExpectedNumberOrString { actual }
    }

    /// Constructs `EvalexprError::ExpectedBoolean{actual}`.
    pub fn expected_boolean(actual: Value) -> Self {
        Error::ExpectedBoolean { actual }
    }

    /// Constructs `EvalexprError::ExpectedTuple{actual}`.
    pub fn expected_tuple(actual: Value) -> Self {
        Error::ExpectedTuple { actual }
    }

    /// Constructs `EvalexprError::ExpectedFixedLenTuple{expected_len, actual}`.
    pub fn expected_fixed_len_tuple(expected_len: usize, actual: Value) -> Self {
        Error::ExpectedFixedLenTuple {
            expected_len,
            actual,
        }
    }

    /// Constructs `EvalexprError::ExpectedEmpty{actual}`.
    pub fn expected_empty(actual: Value) -> Self {
        Error::ExpectedEmpty { actual }
    }

    /// Constructs `EvalexprError::ExpectedEmpty{actual}`.
    pub fn expected_map(actual: Value) -> Self {
        Error::ExpectedMap { actual }
    }

    /// Constructs `EvalexprError::ExpectedEmpty{actual}`.
    pub fn expected_table(actual: Value) -> Self {
        Error::ExpectedTable { actual }
    }

    /// Constructs `EvalexprError::ExpectedEmpty{actual}`.
    pub fn expected_function(actual: Value) -> Self {
        Error::ExpectedFunction { actual }
    }

    /// Constructs an error that expresses that the type of `expected` was expected, but `actual` was found.
    #[allow(unused)]
    pub(crate) fn expected_type(expected: &Value, actual: Value) -> Self {
        match ValueType::from(expected) {
            ValueType::String => Self::expected_string(actual),
            ValueType::Int => Self::expected_int(actual),
            ValueType::Float => Self::expected_float(actual),
            ValueType::Boolean => Self::expected_boolean(actual),
            ValueType::Tuple => Self::expected_tuple(actual),
            ValueType::Empty => Self::expected_empty(actual),
            ValueType::Map => Self::expected_map(actual),
            ValueType::Table => Self::expected_table(actual),
            ValueType::Function => todo!(),
        }
    }

    pub(crate) fn unmatched_partial_token(
        first: PartialToken,
        second: Option<PartialToken>,
    ) -> Self {
        Error::UnmatchedPartialToken { first, second }
    }

    pub(crate) fn addition_error(augend: Value, addend: Value) -> Self {
        Error::AdditionError { augend, addend }
    }

    pub(crate) fn subtraction_error(minuend: Value, subtrahend: Value) -> Self {
        Error::SubtractionError {
            minuend,
            subtrahend,
        }
    }

    pub(crate) fn negation_error(argument: Value) -> Self {
        Error::NegationError { argument }
    }

    pub(crate) fn multiplication_error(multiplicand: Value, multiplier: Value) -> Self {
        Error::MultiplicationError {
            multiplicand,
            multiplier,
        }
    }

    pub(crate) fn division_error(dividend: Value, divisor: Value) -> Self {
        Error::DivisionError { dividend, divisor }
    }

    pub(crate) fn modulation_error(dividend: Value, divisor: Value) -> Self {
        Error::ModulationError { dividend, divisor }
    }

    /// Constructs `EvalexprError::InvalidRegex(regex)`
    pub fn invalid_regex(regex: String, message: String) -> Self {
        Error::InvalidRegex { regex, message }
    }
}

/// Returns `Ok(())` if the actual and expected parameters are equal, and `Err(Error::WrongOperatorArgumentAmount)` otherwise.
pub(crate) fn expect_operator_argument_amount(actual: usize, expected: usize) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::wrong_operator_argument_amount(actual, expected))
    }
}

/// Returns `Ok(())` if the actual and expected parameters are equal, and `Err(Error::WrongFunctionArgumentAmount)` otherwise.
pub fn expect_function_argument_amount(actual: usize, expected: usize) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::wrong_function_argument_amount(actual, expected))
    }
}

/// Returns `Ok(())` if the given value is a string or a numeric
pub fn expect_number_or_string(actual: &Value) -> Result<()> {
    match actual {
        Value::String(_) | Value::Float(_) | Value::Integer(_) => Ok(()),
        _ => Err(Error::expected_number_or_string(actual.clone())),
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::Error::*;
        match self {
            WrongOperatorArgumentAmount { expected, actual } => write!(
                f,
                "An operator expected {} arguments, but got {}.",
                expected, actual
            ),
            WrongFunctionArgumentAmount { expected, actual } => write!(
                f,
                "A function expected {} arguments, but got {}.",
                expected, actual
            ),
            ExpectedString { actual } => {
                write!(f, "Expected a Value::String, but got {:?}.", actual)
            }
            ExpectedInt { actual } => write!(f, "Expected a Value::Int, but got {:?}.", actual),
            ExpectedFloat { actual } => write!(f, "Expected a Value::Float, but got {:?}.", actual),
            ExpectedNumber { actual } => write!(
                f,
                "Expected a Value::Float or Value::Int, but got {:?}.",
                actual
            ),
            ExpectedNumberOrString { actual } => write!(
                f,
                "Expected a Value::Number or a Value::String, but got {:?}.",
                actual
            ),
            ExpectedBoolean { actual } => {
                write!(f, "Expected a Value::Boolean, but got {:?}.", actual)
            }
            ExpectedTuple { actual } => write!(f, "Expected a Value::Tuple, but got {:?}.", actual),
            ExpectedFixedLenTuple {
                expected_len,
                actual,
            } => write!(
                f,
                "Expected a Value::Tuple of len {}, but got {:?}.",
                expected_len, actual
            ),
            ExpectedEmpty { actual } => write!(f, "Expected a Value::Empty, but got {:?}.", actual),
            ExpectedMap { actual } => write!(f, "Expected a Value::Map, but got {:?}.", actual),
            ExpectedTable { actual } => write!(f, "Expected a Value::Table, but got {:?}.", actual),
            ExpectedFunction { actual } => {
                write!(f, "Expected Value::Function, but got {:?}.", actual)
            }
            AppendedToLeafNode => write!(f, "Tried to append a node to a leaf node."),
            PrecedenceViolation => write!(
                f,
                "Tried to append a node to another node with higher precedence."
            ),
            VariableIdentifierNotFound(identifier) => write!(
                f,
                "Variable identifier is not bound to anything by context: {:?}.",
                identifier
            ),
            FunctionIdentifierNotFound(identifier) => write!(
                f,
                "Function identifier is not bound to anything by context: {:?}.",
                identifier
            ),
            TypeError { expected, actual } => {
                write!(f, "Expected one of {:?}, but got {:?}.", expected, actual)
            }
            WrongTypeCombination { operator, actual } => write!(
                f,
                "The operator {:?} was called with a wrong combination of types: {:?}",
                operator, actual
            ),
            UnmatchedLBrace => write!(f, "Found an unmatched opening parenthesis '('."),
            UnmatchedRBrace => write!(f, "Found an unmatched closing parenthesis ')'."),
            MissingOperatorOutsideOfBrace { .. } => write!(
                f,
                "Found an opening parenthesis that is preceded by something that does not take \
                 any arguments on the right, or found a closing parenthesis that is succeeded by \
                 something that does not take any arguments on the left."
            ),
            UnmatchedPartialToken { first, second } => {
                if let Some(second) = second {
                    write!(
                        f,
                        "Found a partial token '{}' that should not be followed by '{}'.",
                        first, second
                    )
                } else {
                    write!(
                        f,
                        "Found a partial token '{}' that should be followed by another partial \
                         token.",
                        first
                    )
                }
            }
            AdditionError { augend, addend } => write!(f, "Error adding {} + {}", augend, addend),
            SubtractionError {
                minuend,
                subtrahend,
            } => write!(f, "Error subtracting {} - {}", minuend, subtrahend),
            NegationError { argument } => write!(f, "Error negating -{}", argument),
            MultiplicationError {
                multiplicand,
                multiplier,
            } => write!(f, "Error multiplying {} * {}", multiplicand, multiplier),
            DivisionError { dividend, divisor } => {
                write!(f, "Error dividing {} / {}", dividend, divisor)
            }
            ModulationError { dividend, divisor } => {
                write!(f, "Error modulating {} % {}", dividend, divisor)
            }
            InvalidRegex { regex, message } => write!(
                f,
                "Regular expression {:?} is invalid: {:?}",
                regex, message
            ),
            ContextNotMutable => write!(f, "Cannot manipulate context"),
            BuiltinFunctionsCannotBeEnabled => {
                write!(f, "This context does not allow enabling builtin functions")
            }
            BuiltinFunctionsCannotBeDisabled => {
                write!(f, "This context does not allow disabling builtin functions")
            }
            IllegalEscapeSequence(string) => write!(f, "Illegal escape sequence: {}", string),
            FunctionFailure(message) => write!(f, "Function failure: {}", message),
            CustomMessage(message) => write!(f, "Error: {}", message),
            WrongColumnAmount { expected, actual } => write!(
                f,
                "Wrong number of columns for this table. Expected {expected}, found {actual}."
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, Value, ValueType};

    /// Tests whose only use is to bring test coverage of trivial lines up, like trivial constructors.
    #[test]
    fn trivial_coverage_tests() {
        assert_eq!(
            Error::type_error(Value::Integer(3), vec![ValueType::String]),
            Error::TypeError {
                actual: Value::Integer(3),
                expected: vec![ValueType::String]
            }
        );
        assert_eq!(
            Error::expected_type(&Value::String("abc".to_string()), Value::Empty),
            Error::expected_string(Value::Empty)
        );
        assert_eq!(
            Error::expected_type(&Value::Boolean(false), Value::Empty),
            Error::expected_boolean(Value::Empty)
        );
        assert_eq!(
            Error::expected_type(&Value::List(vec![]), Value::Empty),
            Error::expected_tuple(Value::Empty)
        );
        assert_eq!(
            Error::expected_type(&Value::Empty, Value::String("abc".to_string())),
            Error::expected_empty(Value::String("abc".to_string()))
        );
    }
}
