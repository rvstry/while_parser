#[derive(Debug, Eq, PartialEq)]
pub enum LexError {
    WhileError,
}

#[derive(Debug)]
pub enum ParseError {
    // ParseError,
    TokenError,
    ParseProgError,
    ParseStmtError,
    ParseStmtsError,
    ParseBExpError,
    ParseBExpsError,
    ParseBFacError,
    ParseBFacsError,
    ParseBNegError,
    ParseBRelError,
    ParseBRelsError,
    ParseAExpError,
    ParseAExpsError,
    ParseAFacError,
    ParseAFacsError,
    ParseAtomError,
}
