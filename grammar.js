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

  // Define tokens that the external scanner will handle
  // The order here must match the enum in scanner.c
  externals: $ => [
    $._rfc3986_uri,    // 0: for namespace-stmt
  ],

  rules: {
    /**
     * @description Try best to follow the YANG grammar definition
     * @see {@link https://www.rfc-editor.org/rfc/rfc6020#section-12 RFC 6020, Section 12, "YANG ABNF Grammar"}
     * @see {@link https://www.rfc-editor.org/rfc/rfc7950#section-14 RFC 7950, Section 14, "YANG ABNF Grammar"} 
    */
    yang: $ => choice($.module_stmt, $.submodule_stmt),

    /** module-stmt         = optsep module-keyword sep identifier-arg-str
                             optsep
                             "{" stmtsep
                                 module-header-stmts
                                 linkage-stmts
                                 meta-stmts
                                 revision-stmts
                                 body-stmts
                             "}" optsep */
    module_stmt: $ => Statement('module', $._identifier_arg_str, $._module_block),
    _module_block: $ => Block(repeat(choice(
      $._module_header,
      $._meta_stmt,
      $.revision_stmt))),

    /** module-header-stmts = ;; these stmts can appear in any order
                         [yang-version-stmt stmtsep]
                          namespace-stmt stmtsep
                          prefix-stmt stmtsep */
    _module_header: $ => choice($.yang_version, $.namespace_stmt, $.prefix_stmt),

    /** prefix-stmt         = prefix-keyword sep prefix-arg-str
                         optsep stmtend */
    prefix_stmt: $ => NonBlockStmt('prefix', $._prefix_arg_str),

    /** submodule-stmt      = optsep submodule-keyword sep identifier-arg-str
                         optsep
                         "{" stmtsep
                             submodule-header-stmts
                             linkage-stmts
                             meta-stmts
                             revision-stmts
                             body-stmts
                         "}" optsep*/
    submodule_stmt: $ => Statement('submodule', $._identifier_arg_str, $._submodule_block),
    _submodule_block: $ => Block(repeat(
      choice(
        $._submodule_header,
        $._meta_stmt,
        $.revision_stmt))),

    /** submodule-header-stmts =
                         ;; these stmts can appear in any order
                         [yang-version-stmt stmtsep]
                          belongs-to-stmt stmtsep */
    _submodule_header: $ => choice($.yang_version, $.belongs_to),

    /** belongs-to-stmt     = belongs-to-keyword sep identifier-arg-str
                         optsep
                         "{" stmtsep
                             prefix-stmt stmtsep
                         "}" */
    belongs_to: $ => NonBlockStmt('belongs-to', $._identifier_arg_str),

    /** yang-version-stmt   = yang-version-keyword sep yang-version-arg-str
                         optsep stmtend */
    yang_version: $ => NonBlockStmt('yang-version', $._yang_version_arg_str),
    _yang_version_arg_str: $ => ArgStr($._yang_versions),
    _yang_versions: _ => {
      /**
       * @todo find a rule to report better error message for invalid versions
       * @file 003_yang_version.rs */
      const versions = /[1]|[1][\.][1]/;
      return token(versions);
    },

    /** namespace-stmt      = namespace-keyword sep uri-str optsep stmtend */
    namespace_stmt: $ => NonBlockStmt('namespace', alias(choice($._rfc3986_uri, $.identifier), $.uri_str)),

    /** linkage-stmts       = ;; these stmts can appear in any order
                         *import-stmt
                         *include-stmt */
    _linkage_stmt: $ => choice($.import_stmt, $.include_stmt),

    /** import-stmt         = import-keyword sep identifier-arg-str optsep
                         "{" stmtsep
                             ;; these stmts can appear in any order
                             prefix-stmt
                             [revision-date-stmt]
                             [description-stmt]
                             [reference-stmt]
                         "}" stmtsep */
    import_stmt: $ => Statement('import', $._identifier_arg_str, Block(repeat(
      choice(
        $.prefix_stmt,
        $.revision_date,
        $.description,
        $.reference)))),

    /** revision-date-stmt  = revision-date-keyword sep revision-date stmtend */
    revision_date: $ => NonBlockStmt('revision-date', $._date_arg_str),

    /** include-stmt        = include-keyword sep identifier-arg-str optsep
                         (";" /
                          "{" stmtsep
                              ;; these stmts can appear in any order
                              [revision-date-stmt]
                              [description-stmt]
                              [reference-stmt]
                          "}") stmtsep */
    include_stmt: $ => Statement('include', $._identifier_arg_str, Block(repeat(
      choice(
        $.revision_date,
        $.description,
        $.reference)))),

    /** meta-stmts          = ;; these stmts can appear in any order
                         [organization-stmt stmtsep]
                         [contact-stmt stmtsep]
                         [description-stmt stmtsep]
                         [reference-stmt stmtsep]*/
    _meta_stmt: $ => choice($.organization, $.contact, $.description, $.reference),

    /** organization-stmt   = organization-keyword sep string
                         optsep stmtend*/
    organization: $ => NonBlockStmt('organization', $.string),

    /** contact-stmt        = contact-keyword sep string optsep stmtend*/
    contact: $ => NonBlockStmt('contact', $.string),

    /** description-stmt    = description-keyword sep string optsep
                         stmtend*/
    description: $ => NonBlockStmt('description', $.string),

    /** reference-stmt      = reference-keyword sep string optsep stmtend*/
    reference: $ => NonBlockStmt('reference', $.string),

    /** revision-stmts      = *(revision-stmt stmtsep)*/
    /** revision-stmt       = revision-keyword sep revision-date optsep
                         (";" /
                          "{" stmtsep
                              [description-stmt stmtsep]
                              [reference-stmt stmtsep]
                          "}")*/
    revision_stmt: $ => Statement('revision', $._date_arg_str, Block(repeat(choice($.description, $.reference)))),

    /** revision-date       =  date-arg-str*/
    _date_arg_str: $ => ArgStr($.date_str),
    date_str: _ => {
      const date_regex = /[1-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]/;
      return token(date_regex);
    },

    // Copied from "tree-sitter-javascript":
    // https://github.com/tree-sitter/tree-sitter-javascript/blob/2c5b138ea488259dbf11a34595042eb261965259/grammar.js#L907
    comment: $ => token(choice(
      seq('//', /(\\(.|\r?\n)|[^\\\n])*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
        '/')
    )),

    _prefix_arg_str: $ => $._identifier_arg_str,

    _identifier_arg_str: $ => choice(
      seq('"', $.identifier, '"'),
      $.identifier
    ),

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

    _single_quoted_string: $ => SingleQuoted(
      repeat1(choice(
        $._unescaped_string1,
        $._escape_sequence,
      )))
    ,

    _double_quoted_string: $ => DoubleQuoted(
      repeat1(choice(
        $._unescaped_string2,
        $._escape_sequence,
      ))),

    _empty_string: _ => choice('""', "''"),

    _quoted_string: $ => choice(
      $._single_quoted_string,
      $._double_quoted_string,
      $._empty_string
    ),

    string: $ => choice($._quoted_string, $.identifier),
  }
});

/**
 * Creates a single-quoted rule
 *
 * @param {Rule} rule YANG rule
 * @returns {Rule} 
 */
function SingleQuoted(rule) {
  return seq("'", rule, "'");
}

/**
 * Creates a double-quoted rule
 *
 * @param {Rule} rule YANG rule
 * @returns {Rule} 
 */
function DoubleQuoted(rule) {
  return seq('"', rule, '"');
}

/**
 * Creates a YANG argument string rule
 *
 * @param {Rule} rule YANG rule
 * @returns {Rule} 
 */
function ArgStr(rule) {
  return choice(
    SingleQuoted(rule),
    DoubleQuoted(rule),
    rule,
  );
}

/**
 * Creates a YANG statement with 0 arguments. E.g., "input" | "output"
 *
 * @param {Rule} rule YANG rule
 * @returns {Rule} 
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
