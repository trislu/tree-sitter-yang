/**
 * @file Yang grammar for tree-sitter
 * @author trislu <lukai@mail.ustc.edu.cn>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const min_keyword = 'min';
const max_keyword = 'max';

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

  conflicts: $ => [
    [$._numerical_restrictions, $._decimal64_specification],
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
      $._linkage_stmt,
      $._meta_stmt,
      $.revision_stmt,
      $._body_stmt))),

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
        $._linkage_stmt,
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
    _yang_version_arg_str: $ => ArgStr($._yang_version_val),
    _yang_version_val: _ => {
      /**
       * @todo find a rule to report better error message for invalid version values
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
    include_stmt: $ => Statement('include', $._identifier_arg_str, OptionalBlock(repeat(
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
    revision_stmt: $ => Statement('revision', $._date_arg_str, OptionalBlock(repeat(choice($.description, $.reference)))),
    /** revision-date       =  date-arg-str*/
    _date_arg_str: $ => ArgStr($.date_str),
    date_str: _ => {
      const date_regex = /[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]/;
      return token(date_regex);
    },

    /** body-stmts          = *(extension-stmt /
                           feature-stmt /
                           identity-stmt /
                           typedef-stmt /
                           grouping-stmt /
                           data-def-stmt /
                           augment-stmt /
                           rpc-stmt /
                           notification-stmt /
                           deviation-stmt)*/
    _body_stmt: $ => choice(
      $.extension_stmt,
      $.feature_stmt,
      $.identity_stmt,
      $.typedef_stmt,
    ),

    /** extension-stmt      = extension-keyword sep identifier-arg-str optsep
                         (";" /
                          "{" stmtsep
                              ;; these stmts can appear in any order
                              [argument-stmt]
                              [status-stmt]
                              [description-stmt]
                              [reference-stmt]
                          "}") stmtsep*/
    extension_stmt: $ => Statement(
      'extension',
      $._identifier_arg_str,
      OptionalBlock(repeat(choice($.argument_stmt, $.status_stmt, $.description, $.reference)))),

    /** argument-stmt       = argument-keyword sep identifier-arg-str optsep
                         (";" /
                          "{" stmtsep
                              [yin-element-stmt stmtsep]
                          "}")*/
    argument_stmt: $ => Statement('argument', $._identifier_arg_str, OptionalBlock(optional($.yin_element))),

    /** yin-element-stmt    = yin-element-keyword sep yin-element-arg-str
                         stmtend*/
    yin_element: $ => NonBlockStmt('yin-element', $._yin_element_arg_str),
    /** yin-element-arg-str = < a string that matches the rule
                           yin-element-arg >
        yin-element-arg     = true-keyword / false-keyword*/
    _yin_element_arg_str: $ => ArgStr($._boolean),

    /** status-stmt         = status-keyword sep status-arg-str stmtend
        status-arg-str      = < a string that matches the rule
                           status-arg >
        status-arg          = current-keyword /
                              obsolete-keyword /
                              deprecated-keyword
    */
    status_stmt: $ => NonBlockStmt('status', $._status_arg_str),
    _status_arg_str: $ => ArgStr(choice(
      'current', 'obsolete', 'deprecated'
    )),

    /** feature-stmt        = feature-keyword sep identifier-arg-str optsep
                         (";" /
                          "{" stmtsep
                              ;; these stmts can appear in any order
                              *(if-feature-stmt stmtsep)
                              [status-stmt stmtsep]
                              [description-stmt stmtsep]
                              [reference-stmt stmtsep]
                          "}")
        if-feature-stmt     = if-feature-keyword sep identifier-ref-arg-str
                         optsep stmtend*/
    feature_stmt: $ => Statement('feature', $._identifier_arg_str,
      OptionalBlock(repeat(choice(
        $.if_feature_stmt,
        $.status_stmt,
        $.description,
        $.reference)))),
    if_feature_stmt: $ => NonBlockStmt('if-feature', $._identifier_arg_str),

    /** identity-stmt       = identity-keyword sep identifier-arg-str optsep
                         (";" /
                          "{" stmtsep
                              ;; these stmts can appear in any order
                              [base-stmt stmtsep]
                              [status-stmt stmtsep]
                              [description-stmt stmtsep]
                              [reference-stmt stmtsep]
                          "}")
        base-stmt           = base-keyword sep identifier-ref-arg-str
                         optsep stmtend */
    identity_stmt: $ => Statement('identity', $._identifier_arg_str,
      OptionalBlock(repeat(choice(
        $.base_stmt,
        $.status_stmt,
        $.description,
        $.reference)))
    ),
    base_stmt: $ => NonBlockStmt('base', $._identifier_ref_arg_str),

    /** typedef-stmt        = typedef-keyword sep identifier-arg-str optsep
                         "{" stmtsep
                             ;; these stmts can appear in any order
                             type-stmt
                             [units-stmt]
                             [default-stmt]
                             [status-stmt]
                             [description-stmt]
                             [reference-stmt]
                          "}" stmtsep */
    typedef_stmt: $ => Statement('typedef', $._identifier_arg_str,
      Block(repeat(choice(
        $.type_stmt,
        $.units_stmt,
        $.default_stmt,
        $.status_stmt,
        $.description,
        $.reference
      )))),

    /** default-stmt        = default-keyword sep string stmtend*/
    default_stmt: $ => NonBlockStmt('default', $.string),

    /** units-stmt          = units-keyword sep string optsep stmtend*/
    units_stmt: $ => NonBlockStmt('units', $.string),

    /** type-stmt           = type-keyword sep identifier-ref-arg-str optsep
                         (";" /
                          "{" stmtsep
                              type-body-stmts
                          "}")*/
    type_stmt: $ => Statement('type', $._identifier_ref_arg_str, OptionalBlock($._type_body_stmts)),

    /** type-body-stmts     = numerical-restrictions /
                         decimal64-specification /
                         string-restrictions /
                         enum-specification /
                         leafref-specification /
                         identityref-specification /
                         instance-identifier-specification /
                         bits-specification /
                         union-specification
    */
    _type_body_stmts: $ => choice(
      $._numerical_restrictions,
      $._decimal64_specification,
      $._string_restrictions,
      $._enum_specification,
      $._leafref_specification,
      $._identityref_specification,
      $._instance_identifier_specification,
      $._bits_specification,
      $._union_specification,
    ),

    /** numerical-restrictions = range-stmt stmtsep*/
    _numerical_restrictions: $ => $.range_stmt,
    /** range-stmt          = range-keyword sep range-arg-str optsep
                         (";" /
                          "{" stmtsep
                              ;; these stmts can appear in any order
                              [error-message-stmt stmtsep]
                              [error-app-tag-stmt stmtsep]
                              [description-stmt stmtsep]
                              [reference-stmt stmtsep]
                           "}")*/
    range_stmt: $ => Statement('range', $._range_arg_str,
      OptionalBlock(repeat(choice(
        $.error_message_stmt,
        $.error_app_tag_stmt,
        $.description,
        $.reference
      )))),

    /** error-message-stmt  = error-message-keyword sep string stmtend
        error-app-tag-stmt  = error-app-tag-keyword sep string stmtend */
    error_message_stmt: $ => NonBlockStmt('error-message', $.string),
    error_app_tag_stmt: $ => NonBlockStmt('error-app-tag', $.string),

    /** range-arg           = range-part *(optsep "|" optsep range-part)
        range-part          = range-boundary
                              [optsep ".." optsep range-boundary]
        range-boundary      = min-keyword / max-keyword /
                              integer-value / decimal-value
    */
    _range_arg_str: $ => ArgStr($._range_arg),
    _range_arg: $ => BarSep1($._range_part),
    _range_part: $ => seq($._range_boundary, optional(seq('..', $._range_boundary))),
    _range_boundary: $ => choice(min_keyword, max_keyword, $.integer_value),

    /** @todo rfc7950
     * decimal64-specification = ;; these stmts can appear in any order
                             fraction-digits-stmt
                             [range-stmt] */
    _decimal64_specification: $ => choice(
      seq($.range_stmt, $.fraction_digits_stmt),
      seq($.fraction_digits_stmt, optional($.range_stmt))
    ),

    /** fraction-digits-stmt = fraction-digits-keyword sep
                          fraction-digits-arg-str stmtend */
    fraction_digits_stmt: $ => NonBlockStmt('fraction-digits', $._fraction_digits_arg_str),
    _fraction_digits_arg_str: $ => ArgStr($._fraction_digits_arg),
    _fraction_digits_arg: _ => {
      const fraction_digits = choice(
        /[0-9]/,        // 0-9
        /1[0-8]/        // 10-18
      );
      // or just let parser users handle the semantic value check?
      // const fraction_digits = /[0-9]{1,2}/;
      return token(fraction_digits);
    },

    /** string-restrictions = ;; these stmts can appear in any order
                         [length-stmt stmtsep]
                         *(pattern-stmt stmtsep) */
    _string_restrictions: $ => choice(
      seq($.length_stmt, repeat($.pattern_stmt)),
      seq(repeat1($.pattern_stmt), $.length_stmt),
      repeat1($.pattern_stmt)
    ),

    /** length-stmt         = length-keyword sep length-arg-str optsep
                         (";" /
                          "{" stmtsep
                              ;; these stmts can appear in any order
                              [error-message-stmt stmtsep]
                              [error-app-tag-stmt stmtsep]
                              [description-stmt stmtsep]
                              [reference-stmt stmtsep]
                           "}") 
        ;; Lengths
        length-arg-str      = < a string that matches the rule
                                length-arg >
        length-arg          = length-part *(optsep "|" optsep length-part)
        length-part         = length-boundary
                              [optsep ".." optsep length-boundary]
        length-boundary     = min-keyword / max-keyword /
                              non-negative-integer-value */
    length_stmt: $ => Statement('length', $._length_arg_str,
      OptionalBlock(repeat(choice(
        $.error_message_stmt,
        $.error_app_tag_stmt,
        $.description,
        $.reference
      )))),

    _length_arg_str: $ => ArgStr($._length_arg),
    _length_arg: $ => BarSep1($._length_part),
    _length_part: $ => seq($._length_boundary, optional(seq('..', $._length_boundary))),
    _length_boundary: $ => choice(min_keyword, max_keyword, $._non_negative_integer_value),

    /** pattern-stmt        = pattern-keyword sep string optsep
                         (";" /
                          "{" stmtsep
                              ;; these stmts can appear in any order
                              [modifier-stmt]
                              [error-message-stmt stmtsep]
                              [error-app-tag-stmt stmtsep]
                              [description-stmt stmtsep]
                              [reference-stmt stmtsep]
                           "}")
    */
    pattern_stmt: $ => Statement('pattern', $.string,
      OptionalBlock(repeat(choice(
        $.modifier_stmt, // rfc7950 only
        $.error_message_stmt,
        $.error_app_tag_stmt,
        $.description,
        $.reference
      )))),

    /** @note this is rfc7950 only
        modifier-stmt       = modifier-keyword sep modifier-arg-str stmtend
        modifier-arg-str    = < a string that matches the rule >
                              < modifier-arg >
        modifier-arg        = invert-match-keyword*/
    modifier_stmt: $ => NonBlockStmt('modifier', $._modifier_arg_str),
    _modifier_arg_str: $ => ArgStr($._invert_match_keyword),
    _invert_match_keyword: _ => 'invert-match',

    /** enum-specification  = 1*enum-stmt
        enum-stmt           = enum-keyword sep string optsep
                              (";" /
                                "{" stmtsep
                                    ;; these stmts can appear in any order
                                    *if-feature-stmt
                                    [value-stmt]
                                    [status-stmt]
                                    [description-stmt]
                                    [reference-stmt]
                                "}") stmtsep */
    _enum_specification: $ => repeat1($.enum_stmt),
    enum_stmt: $ => Statement('enum', $._enum_arg_str,
      OptionalBlock(repeat(choice(
        $.if_feature_stmt,
        $.value_stmt,
        $.status_stmt,
        $.description,
        $.reference
      )))
    ),
    _enum_arg_str: $ => $.string,
    /** value-stmt          = value-keyword sep integer-value-str stmtend
        integer-value-str   = < a string that matches the rule >
                              < integer-value >*/
    value_stmt: $ => NonBlockStmt('value', $._value_arg_str),
    _value_arg_str: $ => ArgStr($.integer_value),

    /** leafref-specification =
                         ;; these stmts can appear in any order
                         path-stmt
                         [require-instance-stmt]

        path-stmt           = path-keyword sep path-arg-str stmtend
        path-arg-str        = < a string that matches the rule >
                         < path-arg >

        path-arg            = absolute-path / relative-path

        absolute-path       = 1*("/" (node-identifier *path-predicate))

        relative-path       = 1*("../") descendant-path

        descendant-path     = node-identifier
                              [*path-predicate absolute-path]

        path-predicate      = "[" *WSP path-equality-expr *WSP "]"

        path-equality-expr  = node-identifier *WSP "=" *WSP path-key-expr

        path-key-expr       = current-function-invocation *WSP "/" *WSP
                              rel-path-keyexpr
        require-instance-stmt = require-instance-keyword sep
                                  require-instance-arg-str stmtend

        require-instance-arg-str = < a string that matches the rule >
                                    < require-instance-arg >

        require-instance-arg = true-keyword / false-keyword */
    _leafref_specification: $ => choice(
      seq($.path_stmt, optional($.require_instance_stmt)),
      seq($.require_instance_stmt, $.path_stmt)
    ),

    path_stmt: $ => NonBlockStmt('path', $._path_arg_str),
    _path_arg_str: $ => ArgStr($._path_arg),
    _path_arg: $ => choice($._absolute_path, $._relative_path),

    _absolute_path: $ => repeat1(seq(
      '/',
      seq($.node_identifier, repeat($._path_predicate))
    )),

    node_identifier: $ => seq(
      $.identifier,
      optional(seq(':', $.identifier))
    ),

    _relative_path: $ => seq(
      repeat1('../'),
      $._descendant_path
    ),
    _descendant_path: $ => seq(
      $.node_identifier,
      optional(seq(
        repeat($._path_predicate),
        $._absolute_path
      ))
    ),

    _path_predicate: $ => seq(
      '[',
      repeat(/\s+/),
      $._path_equality_expr,
      repeat(/\s+/),
      ']'
    ),

    _path_equality_expr: $ => seq(
      $.node_identifier,
      repeat(/\s+/),
      '=',
      repeat(/\s+/),
      $._path_key_expr
    ),

    _path_key_expr: $ => seq(
      $._current_function_invocation,
      repeat(/\s+/),
      '/',
      repeat(/\s+/),
      $._rel_path_keyexpr
    ),

    _current_function_invocation: _ => token('current()'),
    _rel_path_keyexpr: $ => seq(
      $.node_identifier,
      repeat($._path_predicate)
    ),

    /** instance-identifier-specification =
                         [require-instance-stmt] */
    _instance_identifier_specification: $ =>
      $.require_instance_stmt, //tree-sitter limitation: optional($.require_instance_stmt) match empty string
    require_instance_stmt: $ => NonBlockStmt('require-instance', $._require_instance_arg_str),
    _require_instance_arg_str: $ => ArgStr($._require_instance_arg),
    _require_instance_arg: $ => $._boolean,

    /** identityref-specification = 1*base-stmt */
    _identityref_specification: $ => repeat1($.base_stmt),

    /** bits-specification  = 1*bit-stmt
        bit-stmt            = bit-keyword sep identifier-arg-str optsep
                              (";" /
                                "{" stmtsep
                                    ;; these stmts can appear in any order
                                    *if-feature-stmt
                                    [position-stmt]
                                    [status-stmt]
                                    [description-stmt]
                                    [reference-stmt]
                                "}") stmtsep
        position-stmt       = position-keyword sep
                              position-value-arg-str stmtend
        position-value-arg-str = < a string that matches the rule >
                                  < position-value-arg >
        position-value-arg  = non-negative-integer-value
    */
    _bits_specification: $ => repeat1($.bit_stmt),
    bit_stmt: $ => Statement(
      'bit',
      $._identifier_arg_str,
      OptionalBlock(repeat(choice(
        $.if_feature_stmt,
        $.position_stmt,
        $.status_stmt,
        $.description,
        $.reference
      )))
    ),
    position_stmt: $ => NonBlockStmt('position', $._position_value_arg_str),
    _position_value_arg_str: $ => ArgStr($._position_value_arg),
    _position_value_arg: $ => $._non_negative_integer_value,

    /** union-specification = 1*type-stmt */
    _union_specification: $ => repeat1($.type_stmt),

    /** integer-value       = ("-" non-negative-integer-value)  /
                          non-negative-integer-value
        non-negative-integer-value = "0" / positive-integer-value
        positive-integer-value = (non-zero-digit *DIGIT)
        non-zero-digit      = %x31-39
        DIGIT               = %x30-39 ; 0-9
    */
    integer_value: $ => seq(optional('-'), $._non_negative_integer_value),
    _non_negative_integer_value: $ => choice('0', $._positive_integer_value),
    _positive_integer_value: $ => seq($._non_zero_digit, repeat($._DIGIT)),
    _non_zero_digit: _ => /[1-9]/,
    _DIGIT: _ => /[0-9]/,

    // Copied from "tree-sitter-javascript":
    // https://github.com/tree-sitter/tree-sitter-javascript/blob/2c5b138ea488259dbf11a34595042eb261965259/grammar.js#L907
    comment: $ => token(choice(
      seq('//', /(\\(.|\r?\n)|[^\\\n])*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
        '/')
    )),

    _prefix_arg_str: $ => ArgStr($._prefix_arg),
    _prefix_arg: $ => $.identifier,

    _identifier_ref_arg_str: $ => ArgStr($._identifier_ref_arg),
    _identifier_ref_arg: $ => seq(
      optional(seq(
        alias($._prefix_arg, $.prefix),
        ':')),
      $.identifier),

    _identifier_arg_str: $ => ArgStr($._identifier_arg),
    _identifier_arg: $ => $.identifier,
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

    _boolean: _ => choice('true', 'false'),
  }
});

/**
 * Creates a rule to match one or more of the rules separated by a bar
 *
 * @param {Rule} rule
 *
 * @returns {SeqRule}
 */
function BarSep1(rule) {
  return seq(rule, repeat(seq('|', rule)));
}

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
 * Creates a YANG statement with a block of sub-statements.
 *
 * @param {Rule} rule YANG rule
 * @returns {Rule}
 */
function Block(rule) {
  return seq('{', rule, '}');
}

/**
 * Creates a YANG statement with 0-1 block of sub-statements.
 *
 * @param {Rule} rule YANG rule
 * @returns {Rule}
 */
function OptionalBlock(rule) {
  return choice(';', Block(rule));
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
 * Creates a YANG statement with empty block field. E.g., "namespace" | "prefix"
 *
 * @param {string} keyword YANG keyword
 * @param {Rule | string} argument argument of the keyword
 * @returns {Rule} YANG statement
 */
function NonBlockStmt(keyword, argument) {
  return seq(
    keyword,
    field('arg', argument),
    choice(';', seq('{', '}')),
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
