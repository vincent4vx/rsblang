/**
 * All units of the code
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /**
     * Represent any symbol, like function name,
     * variable name, etc...
     * Takes as parameter the name
     *
     * Note: symbol are distinct from keyword, where each keyword has its own token
     */
    Symbol(String),

    /**
     * Represent a constant integer value
     */
    Integer(i32),

    /**
     * Represent a "character" token (i.e. value wrapped between single quote)
     * Unlike C like language, in this value allows to have multiple chars,
     * because it uses a word to store this value, so it can store at most 4 chars into it
     * When encounter less than 4 chars, other values will be filled with NULL (0)
     * At least one char is required.
     */
    Char([char; 4]),

    /**
     * Represent a string token (i.e. value wrapped between single quote)
     * The value of this token doesn't include the termination character (0x05)
     */
    String(String),

    /**
     * Represent an operator, can be unary, or binary.
     * The value is the operator string. A string is used because some operators can have multiple
     * characters, like increment "++"
     */
    Operator(String),

    /**
     * Represent the end of a statement (character ;)
     */
    EndOfStatement,

    /**
     * The opening curly brace {
     */
    OpeningBrace,

    /**
     * The closing curly brace }
     */
    ClosingBrace,

    /**
     * The opening parenthesis (
     */
    OpeningParenthesis,

    /**
     * The closing parenthesis )
     */
    ClosingParenthesis,

    /**
     * The opening bracket [
     */
    OpeningBracket,

    /**
     * The closing bracket ]
     */
    ClosingBracket,

    /**
     * The auto keyword. Used to declare variable.
     */
    Auto,

    /**
     * The extrn keyword. Use to declare an external symbol (i.e. symbol not yet found)
     */
    Extern,

    /**
     * The case statement. Use as a branch of a switch statement.
     */
    Case,

    /**
     * The if conditional statement.
     */
    If,

    /**
     * The else statement.
     */
    Else,

    /**
     * The while statement.
     */
    While,

    /**
     * The switch statement.
     */
    Switch,

    /**
     * The goto statement.
     */
    Goto,

    /**
     * The return statement.
     */
    Return,
}
