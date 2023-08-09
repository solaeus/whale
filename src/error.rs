//! Error and Result types.
//!
//! To deal with errors from dependencies, either create a new error variant
//! or use the MacroFailure variant if the error can only occur inside a macro.
use crate::{
    operator::Operator, token::PartialToken, value::value_type::ValueType, value::Value, Node,
};

use std::{fmt, io, time::SystemTimeError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// A row was inserted to a table with the wrong amount of values.
    WrongColumnAmount {
        expected: usize,
        actual: usize,
    },

    /// An operator was called with the wrong amount of arguments.
    ExpectedOperatorArgumentAmount {
        expected: usize,
        actual: usize,
    },

    /// A function was called with the wrong amount of arguments.
    ExpectedFunctionArgumentAmount {
        identifier: String,
        expected: usize,
        actual: usize,
    },

    /// A function was called with the wrong amount of arguments.
    ExpectedAtLeastFunctionArgumentAmount {
        identifier: String,
        minimum: usize,
        actual: usize,
    },

    ExpectedString {
        actual: Value,
    },

    ExpectedInt {
        actual: Value,
    },

    ExpectedFloat {
        actual: Value,
    },

    /// An integer, floating point or value was expected.
    ExpectedNumber {
        actual: Value,
    },

    /// An integer, floating point or string value was expected.
    ExpectedNumberOrString {
        actual: Value,
    },

    ExpectedBoolean {
        actual: Value,
    },

    ExpectedList {
        actual: Value,
    },

    ExpectedFixedLenList {
        expected_len: usize,
        actual: Value,
    },

    ExpectedEmpty {
        actual: Value,
    },

    ExpectedMap {
        actual: Value,
    },

    ExpectedTable {
        actual: Value,
    },

    ExpectedFunction {
        actual: Value,
    },

    /// A string, list, map or table value was expected.
    ExpectedCollection {
        actual: Value,
    },

    /// Tried to append a child to a leaf node.
    /// Leaf nodes cannot have children.
    AppendedToLeafNode(Node),

    /// Tried to append a child to a node such that the precedence of the child
    /// is not higher. This error should never occur. If it does, please file a
    /// bug report.
    PrecedenceViolation,

    /// A `VariableIdentifier` operation did not find its value in the context.
    VariableIdentifierNotFound(String),

    /// A `FunctionIdentifier` operation did not find its value in the context.
    FunctionIdentifierNotFound(String),

    /// A value has the wrong type.
    /// Only use this if there is no other error that describes the expected and
    /// provided types in more detail.
    TypeError {
        /// The expected types.
        expected: &'static [ValueType],
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
    MacroFailure(String),

    /// A custom error explained by its message.
    CustomMessage(String),
}

impl From<csv::Error> for Error {
    fn from(value: csv::Error) -> Self {
        Error::MacroFailure(value.to_string())
    }
}

impl From<json::Error> for Error {
    fn from(value: json::Error) -> Self {
        Error::MacroFailure(value.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::MacroFailure(value.to_string())
    }
}

impl From<git2::Error> for Error {
    fn from(value: git2::Error) -> Self {
        Error::MacroFailure(value.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::MacroFailure(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::MacroFailure(value.to_string())
    }
}

impl From<sys_info::Error> for Error {
    fn from(value: sys_info::Error) -> Self {
        Error::MacroFailure(value.to_string())
    }
}

impl From<SystemTimeError> for Error {
    fn from(value: SystemTimeError) -> Self {
        Error::MacroFailure(value.to_string())
    }
}

impl From<trash::Error> for Error {
    fn from(value: trash::Error) -> Self {
        Error::MacroFailure(value.to_string())
    }
}

impl Error {
    pub(crate) fn expect_operator_argument_amount(actual: usize, expected: usize) -> Result<()> {
        if actual == expected {
            Ok(())
        } else {
            Err(Error::ExpectedOperatorArgumentAmount { expected, actual })
        }
    }

    pub(crate) fn expect_function_argument_amount(
        identifier: &str,
        actual: usize,
        expected: usize,
    ) -> Result<()> {
        if actual == expected {
            Ok(())
        } else {
            Err(Error::ExpectedFunctionArgumentAmount {
                identifier: identifier.to_string(),
                expected,
                actual,
            })
        }
    }

    pub(crate) fn expected_minimum_function_argument_amount(
        identifier: &str,
        actual: usize,
        minimum: usize,
    ) -> Result<()> {
        if actual >= minimum {
            Ok(())
        } else {
            Err(Error::ExpectedAtLeastFunctionArgumentAmount {
                identifier: identifier.to_string(),
                minimum,
                actual,
            })
        }
    }

    pub fn type_error(actual: Value, expected: &'static [ValueType]) -> Self {
        Error::TypeError { actual, expected }
    }

    pub fn wrong_type_combination(operator: Operator, actual: Vec<ValueType>) -> Self {
        Error::WrongTypeCombination { operator, actual }
    }

    pub fn expected_string(actual: Value) -> Self {
        Error::ExpectedString { actual }
    }

    pub fn expected_int(actual: Value) -> Self {
        Error::ExpectedInt { actual }
    }

    pub fn expected_float(actual: Value) -> Self {
        Error::ExpectedFloat { actual }
    }

    pub fn expected_number(actual: Value) -> Self {
        Error::ExpectedNumber { actual }
    }

    pub fn expected_number_or_string(actual: Value) -> Self {
        Error::ExpectedNumberOrString { actual }
    }

    pub fn expected_boolean(actual: Value) -> Self {
        Error::ExpectedBoolean { actual }
    }

    pub fn expected_list(actual: Value) -> Self {
        Error::ExpectedList { actual }
    }

    pub fn expected_fixed_len_list(expected_len: usize, actual: Value) -> Self {
        Error::ExpectedFixedLenList {
            expected_len,
            actual,
        }
    }

    pub fn expected_empty(actual: Value) -> Self {
        Error::ExpectedEmpty { actual }
    }

    pub fn expected_map(actual: Value) -> Self {
        Error::ExpectedMap { actual }
    }

    pub fn expected_table(actual: Value) -> Self {
        Error::ExpectedTable { actual }
    }

    pub fn expected_function(actual: Value) -> Self {
        Error::ExpectedFunction { actual }
    }

    pub fn expected_collection(actual: Value) -> Self {
        Error::ExpectedCollection { actual }
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

/// Returns `Ok(())` if the given value is a string or a numeric.
pub fn expect_number_or_string(actual: &Value) -> Result<()> {
    match actual {
        Value::String(_) | Value::Float(_) | Value::Integer(_) => Ok(()),
        _ => Err(Error::expected_number_or_string(actual.clone())),
    }
}

/// Returns `Ok(())` if the given value is a String, List, Map or Table.
pub fn _expect_collection(actual: &Value) -> Result<()> {
    match actual {
        Value::String(_) | Value::List(_) | Value::Map(_) | Value::Table(_) => Ok(()),
        _ => Err(Error::expected_collection(actual.clone())),
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            ExpectedOperatorArgumentAmount { expected, actual } => write!(
                f,
                "An operator expected {} arguments, but got {}.",
                expected, actual
            ),
            ExpectedFunctionArgumentAmount {
                expected,
                actual,
                identifier,
            } => write!(
                f,
                "{identifier} expected {expected} arguments, but got {actual}.",
            ),
            ExpectedAtLeastFunctionArgumentAmount {
                minimum,
                actual,
                identifier,
            } => write!(
                f,
                "{identifier} expected a minimum of {minimum} arguments, but got {actual}.",
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
            ExpectedList { actual } => write!(f, "Expected a Value::Tuple, but got {:?}.", actual),
            ExpectedFixedLenList {
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
            ExpectedCollection { actual } => {
                write!(
                    f,
                    "Expected a string, list, map or table, but got {:?}.",
                    actual
                )
            }
            AppendedToLeafNode(node) => write!(f, "Syntax error at \"{node}\"."),
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
                write!(
                    f,
                    "Type Error. The value {actual} is not one of the following: {expected:?}.",
                )
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
            MacroFailure(message) => write!(f, "Function failure: {}", message),
            CustomMessage(message) => write!(f, "Error: {}", message),
            WrongColumnAmount { expected, actual } => write!(
                f,
                "Wrong number of columns for this table. Expected {expected}, found {actual}."
            ),
        }
    }
}
