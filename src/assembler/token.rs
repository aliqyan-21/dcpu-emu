enum _TokenType {
    CMD,
    HEX,
    LBRACKET,
    RBRACKET,
    LITERAL,
    LABEL,
}

struct _Token {
    t_type: _TokenType,
    value: String,
}

