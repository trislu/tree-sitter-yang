// scanner.c
#include "tree_sitter/alloc.h"
#include "tree_sitter/parser.h"

#include <ctype.h>  // For isspace
#include <stddef.h> // For size_t
#include <stdint.h> // For uint
#include <string.h> // For memcpy & memset

// These must match the order in `externals` array in grammar.js
enum TokenType {
    RFC3986_URI, // 0
};

typedef struct {
    TSLexer *lex;
    const bool *opt;
} Context;

static const size_t context_size = sizeof(Context);

static inline void context_attach(Context *ctx, TSLexer *lex, const bool *opt) {
    ctx->lex = lex;
    ctx->opt = opt;
}

// --- Standard Tree-sitter Scanner Functions ---

// Create a new scanner instance
void *tree_sitter_yang_external_scanner_create() {
    Context *ctx = (Context *)ts_calloc(1, context_size);
    return ctx;
}

// Destroy the scanner instance
void tree_sitter_yang_external_scanner_destroy(void *payload) {
    Context *ctx = (Context *)payload;
    ts_free(ctx);
}

// Serialize the scanner's state (for incremental parsing)
unsigned tree_sitter_yang_external_scanner_serialize(void *payload,
                                                     char *buffer) {
    memcpy(buffer, payload, context_size);
    return context_size;
}

// Deserialize the scanner's state
void tree_sitter_yang_external_scanner_deserialize(void *payload,
                                                   const char *buffer,
                                                   unsigned length) {
    if (length == context_size) {
        // restore scanner state
        memcpy(payload, buffer, context_size);
    } else {
        // Reset if no state is provided (e.g., initial parse)
        memset(payload, 0, context_size);
    }
}

// --- Helper functions of lexer ---

static inline int32_t lex_nextchar(TSLexer *l) {
    // The current next character in the input stream, represented as a 32-bit
    // unicode code point. see also
    // https://tree-sitter.github.io/tree-sitter/creating-parsers/4-external-scanners.html#scan
    return l->lookahead;
}

static inline void lex_skip(TSLexer *l) {
    // passing "true" to ignore whitespace, see also
    // https://tree-sitter.github.io/tree-sitter/creating-parsers/4-external-scanners.html#scan
    l->advance(l, true);
}

static inline bool lex_advance(TSLexer *l) {
    l->advance(l, false);
    return true;
}

static inline bool lex_eof(TSLexer *l) {
    // A function for determining whether the lexer is at the end of the file.
    // The value of lookahead will be 0 at the end of a file, but this function
    // should be used instead of checking for that value because the 0 or "NUL"
    // value is also a valid character that could be present in the file being
    // parsed.
    return l->eof(l);
}

static inline void lex_emit(TSLexer *l, TSSymbol token) {
    // emit the recognized token from the TokenType enum.
    l->result_symbol = token;
}

static inline bool lex_matchc(TSLexer *l, char c) {
    return !lex_eof(l) && (lex_nextchar(l) == c) && lex_advance(l);
}

/** unused
static bool lex_matchs(TSLexer *l, const char *str) {
    for (size_t i = 0; i < strlen(str); i++) {
        if (!lex_matchc(l, str[i])) {
            return false;
        }
    }
    return true;
}
*/

static bool is_rfc3986_valid_char(int32_t c) {
    // Unreserved: A-Za-z0-9-._~
    if (isalnum(c) || c == '-' || c == '.' || c == '_' || c == '~')
        return true;
    // Reserved: :/?#[]@!$&'()*+,;=
    char *endpos = strchr(":/?#[]@!$&'()*+,;=", c);
    if (NULL == endpos || *endpos == '\0')
        return false;
    return true;
}

static bool scan_rfc3986_uri(Context *ctx) {
    TSLexer *lex = ctx->lex;
    if ('"' != lex_nextchar(lex)) {
        return false;
    }
    lex_advance(lex); // skip start '"'
    while (!lex_eof(lex)) {
        int32_t next_char = lex_nextchar(lex);
        if ('"' == next_char) {
            lex_advance(lex); // skip end '"'
            lex_emit(lex, RFC3986_URI);
            return true;
        }
        if (!is_rfc3986_valid_char(next_char)) {
            break;
        }
        lex_advance(lex);
    }
    return false;
}

static bool recognize_token(Context *ctx, TSLexer *lex,
                            const bool *valid_symbols) {
    // attach params to context
    context_attach(ctx, lex, valid_symbols);

    // Skip any leading whitespace
    while (isspace(lex_nextchar(lex))) {
        lex_skip(lex);
    }
    if (lex_eof(lex)) {
        return false;
    }

    if (valid_symbols[RFC3986_URI]) {
        return scan_rfc3986_uri(ctx);
    }

    // If no external token was matched, return false.
    // The main grammar will then try to match a token.
    return false;
}

// --- Main scanning logic ---
bool tree_sitter_yang_external_scanner_scan(void *payload, TSLexer *lex,
                                            const bool *valid_symbols) {
    Context *ctx = (Context *)payload;
    // see if scanner can recognize an external token
    return recognize_token(ctx, lex, valid_symbols);
}