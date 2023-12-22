

enum InuktutMorpheme {
    P, T, K, H, G, M, N, S, Š, H2, L, J, JJ, 
    Ř, V, R, Q, QQ, NG, NNG, Ł, B, H3, STOP,
    AI, A, AA, I, II, U, UU
}


enum TokenType {
    InuktutMorpheme(InuktutMorpheme),
    Other
}

struct Token<'a> {
    token_type: TokenType,
    data: &'a str
}

fn tokenize_latin(buffer: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::with_capacity(100);
    tokens
}