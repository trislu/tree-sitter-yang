/**
 * @file Yang grammar for tree-sitter
 * @author trislu <lukai@mail.ustc.edu.cn>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

export default grammar({
  name: "yang",

  extras: $ => [
    /\s+/,
    $.comment,
  ],

  rules: {
    /**
     * @description Try best to follow the YANG grammar definition
     * @see {@link https://www.rfc-editor.org/rfc/rfc6020#section-12 RFC 6020, Section 12, "YANG ABNF Grammar"}
     * @see {@link https://www.rfc-editor.org/rfc/rfc7950#section-14 RFC 7950, Section 14, "YANG ABNF Grammar"} 
    */
    yang: $ => choice($.module, $.submodule),

    module: $ => Statement('module', $.identifier, $._module_block),
    _module_block: $ => Block(
      repeat(choice($.yang_version, $.prefix, $.namespace))),
    prefix: $ => NonBlockStmt('prefix', $.string),

    submodule: $ => Statement('submodule', $.identifier, $._submodule_block),
    _submodule_block: $ => Block(repeat(choice($.yang_version, $.belongs_to))),
    belongs_to: $ => NonBlockStmt('belongs-to', $.identifier),

    yang_version: $ => NonBlockStmt('yang-version', '1.1'),
    namespace: $ => NonBlockStmt('namespace', $.string),

    // Copied from "tree-sitter-javascript":
    // https://github.com/tree-sitter/tree-sitter-javascript/blob/2c5b138ea488259dbf11a34595042eb261965259/grammar.js#L907
    comment: $ => token(choice(
      seq('//', /(\\(.|\r?\n)|[^\\\n])*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
        '/')
    )),

    identifier: _ => {
      const alpha_underscore = /[a-zA-Z_]/;
      const alphanumeric = /[a-zA-Z0-9-_.]/;
      return token(seq(alpha_underscore, repeat(alphanumeric)));
    },

    /**
     * @see {@link https://www.rfc-editor.org/rfc/rfc7950#section-6.1.3 Quoting}
     */
    // unescaped string that can be single quoted
    _unescaped_string1: _ => token.immediate(prec(1, /[^'\\]+/)),
    // unescaped string that can be single quoted
    _unescaped_string2: _ => token.immediate(prec(1, /[^"\\]+/)),
    // escaped string
    _escape_sequence: _ => token.immediate(seq(
      '\\',
      choice('n', 't', '"', '\\')
    )),

    _single_quoted_string: $ => seq(
      '\'',
      repeat1(choice(
        $._unescaped_string1,
        $._escape_sequence,
      ))
      ,
      '\'',
    ),

    _double_quoted_string: $ => seq(
      '"',
      repeat1(choice(
        $._unescaped_string2,
        $._escape_sequence,
      )),
      '"',
    ),

    _empty_string: $ => choice('""', "''"),

    _quoted_string: $ => choice(
      $._single_quoted_string,
      $._double_quoted_string,
      $._empty_string
    ),

    string: $ => choice($._quoted_string, $.identifier),
  }
});

/**
 * Creates a YANG statement with 0 arguments. E.g., "input" | "output"
 *
 * @param {Rule} rule YANG rule
 * @returns {Rule} 
 * 
 * @note For now we don't check the existence of block, as 
 */
function Block(rule) {
  return seq('{', rule, '}');
}

/**
 * Creates a YANG statement with no argument field. E.g., "input" | "output"
 *
 * @param {string} keyword YANG keyword
 * @param {Rule} block substatement block of the statement
 * @returns {Rule} YANG statement
 * 
 * @note For now we don't check the existence of block, as 
 */
function NonArgStmt(keyword, block) {
  return seq(keyword, block);
}

/**
 * Creates a YANG statement with no block field. E.g., "namespace" | "prefix"
 *
 * @param {string} keyword YANG keyword
 * @param {Rule | string} argument argument of the keyword
 * @returns {Rule} YANG statement
 * 
 * @note For now we don't check the existence of block, as 
 */
function NonBlockStmt(keyword, argument) {
  return seq(
    keyword,
    field('arg', argument),
    ';'
  );
}

/**
 * Creates a YANG statement
 *
 * @param {string} keyword YANG keyword
 * @param {Rule} argument argument of the keyword
 * @param {Rule} block substatement block of the statement
 * @returns {Rule} YANG statement
 * 
 * @note For now we don't check the existence of block, as 
 */
function Statement(keyword, argument, block) {
  if (!argument) {
    return NonArgStmt(keyword, block);
  }
  if (!block) {
    return NonBlockStmt(keyword, argument);
  }
  return seq(keyword, field('arg', argument), block);
}