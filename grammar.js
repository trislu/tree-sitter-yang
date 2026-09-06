/**
 * @file Yang grammar for tree-sitter
 * @author trislu <lukai@mail.ustc.edu.cn>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const deviate_keyword = 'deviate'

const add_arg = 'add'
const delete_arg = 'delete'
const min_arg = 'min'
const max_arg = 'max'
const not_supported_arg = 'not-supported'
const replace_arg = 'replace'

export default grammar({
  name: "yang",

  extras: $ => [
    /\s+/,
    $.comment,
  ],

  conflicts: $ => [
    [$.unknown_stmt],
    // blame unknown_stmt
    [$.action_stmt],
    [$.anydata_stmt],
    [$.anyxml_stmt],
    [$.argument_stmt],
    [$.augment_stmt],
    // augment-stmt and uses-augment-stmt share the 'augment' keyword.
    [$.augment_arg_str, $.uses_augment_arg_str],
    [$.base_stmt],
    [$.belongs_to_stmt],
    [$.bit_stmt],
    [$.case_stmt],
    [$.choice_stmt],
    [$.config_stmt],
    [$.contact_stmt],
    [$.container_stmt],
    [$.default_stmt],
    [$.description_stmt],
    [$.deviate_add_stmt],
    [$.deviate_delete_stmt],
    [$.deviate_not_supported_stmt],
    [$.deviate_replace_stmt],
    [$.deviation_stmt],
    [$.enum_stmt],
    [$.error_app_tag_stmt],
    [$.error_message_stmt],
    [$.extension_stmt],
    [$.feature_stmt],
    [$.fraction_digits_stmt],
    [$.grouping_stmt],
    [$.identity_stmt],
    [$.if_feature_stmt],
    [$.import_stmt],
    [$.include_stmt],
    [$.input_stmt],
    [$.key_stmt],
    [$.leaf_list_stmt],
    [$.leaf_stmt],
    [$.length_stmt],
    [$.list_stmt],
    [$.mandatory_stmt],
    [$.max_elements_stmt],
    [$.min_elements_stmt],
    [$.modifier_stmt],
    [$.must_stmt],
    [$.namespace_stmt],
    [$.notification_stmt],
    [$.ordered_by_stmt],
    [$.organization_stmt],
    [$.output_stmt],
    [$.path_stmt],
    [$.pattern_stmt],
    [$.position_stmt],
    [$.prefix_stmt],
    [$.presence_stmt],
    [$.range_stmt],
    [$.reference_stmt],
    [$.refine_stmt],
    [$.require_instance_stmt],
    [$.revision_date_stmt],
    [$.revision_stmt],
    [$.rpc_stmt],
    [$.status_stmt],
    [$.typedef_stmt],
    [$.type_stmt],
    [$.unique_stmt],
    [$.units_stmt],
    [$.uses_augment_stmt],
    [$.uses_stmt],
    [$.value_stmt],
    [$.when_stmt],
    [$.yang_version_stmt],
    [$.yin_element_stmt],
    [$._range_boundary, $.decimal_value], // maybe need to set precedences?
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
    module_stmt: $ => Statement(
      alias('module', $.module_keyword),
      alias($._identifier_arg_str, $.module_arg_str),
      Block(
        repeat(choice(
          $._module_header,
          $._linkage_stmt,
          $._meta_stmt,
          $.revision_stmt,
          $._body_stmt))),
      false,
    ),

    /** module-header-stmts = ;; these stmts can appear in any order
                         [yang-version-stmt stmtsep]
                          namespace-stmt stmtsep
                          prefix-stmt stmtsep */
    _module_header: $ => choice(
      $.yang_version_stmt,
      $.namespace_stmt,
      $.prefix_stmt
    ),

    /** prefix-stmt         = prefix-keyword sep prefix-arg-str
                         optsep stmtend */
    prefix_stmt: $ => NonBlockStmt(alias('prefix', $.prefix_keyword), $.prefix_arg_str),

    /** submodule-stmt      = optsep submodule-keyword sep identifier-arg-str
                         optsep
                         "{" stmtsep
                             submodule-header-stmts
                             linkage-stmts
                             meta-stmts
                             revision-stmts
                             body-stmts
                         "}" optsep*/
    submodule_stmt: $ => Statement(alias('submodule', $.submodule_keyword), alias($._identifier_arg_str, $.submodule_arg_str),
      Block(
        repeat(
          choice(
            $._submodule_header,
            $._linkage_stmt,
            $._meta_stmt,
            $.revision_stmt,
            $._body_stmt)),
      ),
      false,
    ),

    /** submodule-header-stmts =
                         ;; these stmts can appear in any order
                         [yang-version-stmt stmtsep]
                          belongs-to-stmt stmtsep */
    _submodule_header: $ => choice($.yang_version_stmt, $.belongs_to_stmt),

    /** belongs-to-stmt     = belongs-to-keyword sep identifier-arg-str
                         optsep
                         "{" stmtsep
                             prefix-stmt stmtsep
                         "}" */
    belongs_to_stmt: $ => Statement(alias('belongs-to', $.belongs_to_keyword), alias($._identifier_arg_str, $.belongs_to_arg_str),
      Block($.prefix_stmt),
      false,
    ),

    /** yang-version-stmt   = yang-version-keyword sep yang-version-arg-str
                         optsep stmtend */
    yang_version_stmt: $ => NonBlockStmt(alias('yang-version', $.yang_version_keyword), $.yang_version_arg_str),
    yang_version_arg_str: $ => ArgStr($._yang_version_val),
    _yang_version_val: _ => {
      /**
       * @todo find a rule to report better error message for invalid version values
       * @file 003_yang_version.rs */
      const versions = /[1]|[1][\.][1]/
      return token(versions)
    },

    /** namespace-stmt      = namespace-keyword sep uri-str optsep stmtend
     * Argument: a quoted string (possibly '+' concatenated) or a bare URI;
     * URI semantics are handled by the grammar's consumers. */
    namespace_stmt: $ => NonBlockStmt(alias('namespace', $.namespace_keyword), $.namespace_arg_str),
    namespace_arg_str: $ => choice(
      seq($._uri_quoted, repeat(seq('+', $.quoted_string))),
      $._uri_str,
    ),
    _uri_quoted: $ => choice($._uri_dq, $._uri_sq),
    _uri_dq: _ => token(seq('"', /[^"]*/, '"')),
    _uri_sq: _ => token(seq("'", /[^']*/, "'")),

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
    import_stmt: $ => Statement(alias('import', $.import_keyword), alias($._identifier_arg_str, $.import_arg_str), Block(repeat(
      choice(
        $.prefix_stmt,
        $.revision_date_stmt,
        $.description_stmt,
        $.reference_stmt)))),

    /** revision-date-stmt  = revision-date-keyword sep revision-date stmtend */
    revision_date_stmt: $ => NonBlockStmt(alias('revision-date', $.revision_date_keyword), alias($._date_arg_str, $.revision_date_arg_str)),

    /** include-stmt        = include-keyword sep identifier-arg-str optsep
                         (";" /
                          "{" stmtsep
                              ;; these stmts can appear in any order
                              [revision-date-stmt]
                              [description-stmt]
                              [reference-stmt]
                          "}") stmtsep */
    include_stmt: $ => Statement(alias('include', $.include_keyword), alias($._identifier_arg_str, $.include_arg_str),
      OptionalBlock(repeat(choice(
        $.revision_date_stmt,
        $.description_stmt,
        $.reference_stmt)))),

    /** meta-stmts          = ;; these stmts can appear in any order
                         [organization-stmt stmtsep]
                         [contact-stmt stmtsep]
                         [description-stmt stmtsep]
                         [reference-stmt stmtsep]*/
    _meta_stmt: $ => choice(
      $.organization_stmt,
      $.contact_stmt,
      $.description_stmt,
      $.reference_stmt
    ),

    /** organization-stmt   = organization-keyword sep string
                         optsep stmtend*/
    organization_stmt: $ => NonBlockStmt(alias('organization', $.organization_keyword), alias(choice($.string, $._bare_word), $.organization_arg_str)),

    /** contact-stmt        = contact-keyword sep string optsep stmtend*/
    contact_stmt: $ => NonBlockStmt(alias('contact', $.contact_keyword), alias(choice($.string, $._bare_word), $.contact_arg_str)),

    /** description-stmt    = description-keyword sep string optsep
                         stmtend*/
    /**
     * @note the description argument is a quoted-string.
     */
    description_stmt: $ => NonBlockStmt(alias('description', $.description_keyword), alias(choice($._concatenated_string, $._bare_word), $.description_arg_str)),

    /** reference-stmt      = reference-keyword sep string optsep stmtend*/
    reference_stmt: $ => NonBlockStmt(alias('reference', $.reference_keyword), alias(choice($.string, $._bare_word), $.reference_arg_str)),

    /** revision-stmts      = *(revision-stmt stmtsep)*/
    /** revision-stmt       = revision-keyword sep revision-date optsep
                         (";" /
                          "{" stmtsep
                              [description-stmt stmtsep]
                              [reference-stmt stmtsep]
                          "}")*/
    revision_stmt: $ => Statement(alias('revision', $.revision_keyword), alias($._date_arg_str, $.revision_arg_str),
      OptionalBlock(repeat(choice(
        $.description_stmt,
        $.reference_stmt)))
    ),
    /** revision-date       =  date-arg-str*/
    _date_arg_str: $ => ArgStr($.date_str),
    date_str: _ => {
      const date_regex = /[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]/
      return token(date_regex)
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
      $.grouping_stmt,
      $._data_def_stmt,
      $.augment_stmt,
      $.rpc_stmt,
      $.notification_stmt,
      $.deviation_stmt,
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
    extension_stmt: $ => Statement(alias('extension', $.extension_keyword), alias($._identifier_arg_str, $.extension_arg_str),
      OptionalBlock(repeat(choice(
        $.argument_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt)))
    ),

    /** argument-stmt       = argument-keyword sep identifier-arg-str optsep
                         (";" /
                          "{" stmtsep
                              [yin-element-stmt stmtsep]
                          "}")*/
    argument_stmt: $ => Statement(alias('argument', $.argument_keyword), alias($._identifier_arg_str, $.argument_arg_str),
      OptionalBlock(optional($.yin_element_stmt))),

    /** yin-element-stmt    = yin-element-keyword sep yin-element-arg-str
                         stmtend*/
    yin_element_stmt: $ => NonBlockStmt(alias('yin-element', $.yin_element_keyword), $.yin_element_arg_str),
    /** yin-element-arg-str = < a string that matches the rule
                           yin-element-arg >
        yin-element-arg     = true-keyword / false-keyword*/
    yin_element_arg_str: $ => ArgStr($.boolean),

    /** status-stmt         = status-keyword sep status-arg-str stmtend
        status-arg-str      = < a string that matches the rule
                           status-arg >
        status-arg          = current-keyword /
                              obsolete-keyword /
                              deprecated-keyword
    */
    status_stmt: $ => NonBlockStmt(alias('status', $.status_keyword), $.status_arg_str),
    status_arg_str: $ => ArgStr(choice(
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
    feature_stmt: $ => Statement(alias('feature', $.feature_keyword), alias($._identifier_arg_str, $.feature_arg_str),
      OptionalBlock(repeat(choice(
        $.if_feature_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt)))),

    if_feature_stmt: $ => NonBlockStmt(alias('if-feature', $.if_feature_keyword), alias(choice($._concatenated_string, $._identifier_ref_arg), $.if_feature_arg_str)),

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
    identity_stmt: $ => Statement(alias('identity', $.identity_keyword), alias($._identifier_arg_str, $.identity_arg_str),
      OptionalBlock(repeat(choice(
        $.if_feature_stmt,
        $.base_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt)))
    ),

    base_stmt: $ => NonBlockStmt(alias('base', $.base_keyword), alias($._identifier_ref_arg_str, $.base_arg_str)),

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
    typedef_stmt: $ => Statement(alias('typedef', $.typedef_keyword), alias($._identifier_arg_str, $.typedef_arg_str),
      Block(repeat(choice(
        $.type_stmt,
        $.units_stmt,
        $.default_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt
      )))),

    /** default-stmt        = default-keyword sep string stmtend
        The argument holds a value valid for the node's type (RFC 7950 §7.6.4).
        A `decimal64` default may be a bare decimal such as `-3.25`
        (§9.3.4: decimal64-value = ["-"] integer-value ["." zero-integer-value]),
        so the numeric alternative is `decimal_value` (it also covers bare
        integers). */
    default_stmt: $ => NonBlockStmt(alias('default', $.default_keyword), alias(choice($.string, $._slash_word, ArgStr($.decimal_value), $._bare_word), $.default_arg_str)),

    /** units-stmt          = units-keyword sep string optsep stmtend*/
    units_stmt: $ => NonBlockStmt(alias('units', $.units_keyword), alias(choice($.string, $._slash_word, $._bare_word), $.units_arg_str)),

    /** type-stmt           = type-keyword sep identifier-ref-arg-str optsep
                         (";" /
                          "{" stmtsep
                              type-body-stmts
                          "}")*/
    type_stmt: $ => Statement(alias('type', $.type_keyword), alias(choice($._identifier_ref_arg_str, $._type_junk_word), $.type_arg_str),
      OptionalBlock(seq(
        repeat($.unknown_stmt),
        optional($._type_body_stmts),
        repeat($.unknown_stmt),
      ))),

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
    range_stmt: $ => Statement(alias('range', $.range_keyword), $.range_arg_str,
      OptionalBlock(repeat(choice(
        $.error_message_stmt,
        $.error_app_tag_stmt,
        $.description_stmt,
        $.reference_stmt
      )))),

    /** error-message-stmt  = error-message-keyword sep string stmtend
        error-app-tag-stmt  = error-app-tag-keyword sep string stmtend */
    error_message_stmt: $ => NonBlockStmt(alias('error-message', $.error_message_keyword), $.string),
    error_app_tag_stmt: $ => NonBlockStmt(alias('error-app-tag', $.error_app_tag_keyword), $.string),

    /** range-arg           = range-part *(optsep "|" optsep range-part)
        range-part          = range-boundary
                              [optsep ".." optsep range-boundary]
        range-boundary      = min-keyword / max-keyword /
                              integer-value / decimal-value
    */
    range_arg_str: $ => ArgStr($._range_arg),
    _range_arg: $ => BarSep1($._range_part),
    _range_part: $ => seq($._range_boundary, optional(seq('..', $._range_boundary))),
    _range_boundary: $ => choice(
      min_arg, max_arg,
      $.integer_value, $.decimal_value
    ),

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
    fraction_digits_stmt: $ => NonBlockStmt(alias('fraction-digits', $.fraction_digits_keyword), $.fraction_digits_arg_str),
    fraction_digits_arg_str: $ => ArgStr($._fraction_digits_arg),
    _fraction_digits_arg: _ => {
      const fraction_digits = choice(
        /[0-9]/,        // 0-9
        /1[0-8]/        // 10-18
      )
      // or just let parser users handle the semantic value check?
      // const fraction_digits = /[0-9]{1,2}/;
      return token(fraction_digits)
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
    length_stmt: $ => Statement(alias('length', $.length_keyword), $.length_arg_str,
      OptionalBlock(repeat(choice(
        $.error_message_stmt,
        $.error_app_tag_stmt,
        $.description_stmt,
        $.reference_stmt
      )))),

    length_arg_str: $ => ArgStr($._length_arg),
    _length_arg: $ => BarSep1($._length_part),
    _length_part: $ => seq($._length_boundary, optional(seq('..', $._length_boundary))),
    _length_boundary: $ => choice(min_arg, max_arg, $._non_negative_integer_value),

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
    pattern_stmt: $ => Statement(alias('pattern', $.pattern_keyword), alias($.string, $.pattern_arg_str),
      OptionalBlock(repeat(choice(
        $.modifier_stmt, // rfc7950 only
        $.error_message_stmt,
        $.error_app_tag_stmt,
        $.description_stmt,
        $.reference_stmt
      )))),

    /** @note this is rfc7950 only
        modifier-stmt       = modifier-keyword sep modifier-arg-str stmtend
        modifier-arg-str    = < a string that matches the rule >
                              < modifier-arg >
        modifier-arg        = invert-match-keyword*/
    modifier_stmt: $ => NonBlockStmt(alias('modifier', $.modifier_keyword), $.modifier_arg_str),
    modifier_arg_str: $ => ArgStr($._invert_match_keyword),
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
    enum_stmt: $ => Statement(alias('enum', $.enum_keyword), $.enum_arg_str,
      OptionalBlock(repeat(choice(
        $.if_feature_stmt,
        $.value_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt
      )))
    ),
    enum_arg_str: $ => choice($.string, $._digit_start_word, $._bare_word),
    /** value-stmt          = value-keyword sep integer-value-str stmtend
        integer-value-str   = < a string that matches the rule >
                              < integer-value >*/
    value_stmt: $ => NonBlockStmt(alias('value', $.value_keyword), $.value_arg_str),
    value_arg_str: $ => ArgStr($.integer_value),

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

    path_stmt: $ => NonBlockStmt(alias('path', $.path_keyword), $.path_arg_str),
    path_arg_str: $ => QuotedOr($, $._path_arg),
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
    require_instance_stmt: $ => NonBlockStmt(alias('require-instance', $.require_instance_keyword), $.require_instance_arg_str),
    require_instance_arg_str: $ => ArgStr($._require_instance_arg),
    _require_instance_arg: $ => $.boolean,

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
      alias('bit', $.bit_keyword), alias($._identifier_arg_str, $.bit_arg_str),
      OptionalBlock(repeat(choice(
        $.if_feature_stmt,
        $.position_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt
      )))
    ),
    position_stmt: $ => NonBlockStmt(alias('position', $.position_keyword), $.position_arg_str),
    position_arg_str: $ => ArgStr($._position_value_arg),
    _position_value_arg: $ => $._non_negative_integer_value,

    /** union-specification = 1*type-stmt */
    _union_specification: $ => repeat1($.type_stmt),

    /** grouping-stmt       = grouping-keyword sep identifier-arg-str optsep
                         (";" /
                          "{" stmtsep
                              ;; these stmts can appear in any order
                              [status-stmt]
                              [description-stmt]
                              [reference-stmt]
                              *(typedef-stmt / grouping-stmt)
                              *data-def-stmt
                              *action-stmt
                              *notification-stmt
                          "}") stmtsep */
    grouping_stmt: $ => Statement(alias('grouping', $.grouping_keyword), alias($._identifier_arg_str, $.grouping_arg_str),
      OptionalBlock(repeat(choice(
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
        $.typedef_stmt,
        $.grouping_stmt,
        $._data_def_stmt,
        $.action_stmt,
        $.notification_stmt,
      )))
    ),

    /** data-def-stmt       = container-stmt /
                              leaf-stmt /
                              leaf-list-stmt /
                              list-stmt /
                              choice-stmt /
                              anydata-stmt /
                              anyxml-stmt /
                              uses-stmt */
    _data_def_stmt: $ => choice(
      $.container_stmt,
      $.leaf_stmt,
      $.leaf_list_stmt,
      $.list_stmt,
      $.choice_stmt,
      $.anydata_stmt,
      $.anyxml_stmt,
      $.uses_stmt,
    ),

    /** container-stmt      = container-keyword sep identifier-arg-str optsep
                         (";" /
                          "{" stmtsep
                              ;; these stmts can appear in any order
                              [when-stmt]
                              *if-feature-stmt
                              *must-stmt
                              [presence-stmt]
                              [config-stmt]
                              [status-stmt]
                              [description-stmt]
                              [reference-stmt]
                              *(typedef-stmt / grouping-stmt)
                              *data-def-stmt
                              *action-stmt
                              *notification-stmt
                          "}") stmtsep */
    container_stmt: $ => Statement(alias('container', $.container_keyword), alias($._identifier_arg_str, $.container_arg_str),
      OptionalBlock(repeat(choice(
        $.when_stmt,
        $.if_feature_stmt,
        $.must_stmt,
        $.presence_stmt,
        $.config_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
        $.typedef_stmt,
        $.grouping_stmt,
        $._data_def_stmt,
        $.action_stmt,
        $.notification_stmt,
      )))
    ),

    /** leaf-stmt           = leaf-keyword sep identifier-arg-str optsep
                             "{" stmtsep
                                ;; these stmts can appear in any order
                                [when-stmt]
                                *if-feature-stmt
                                type-stmt
                                [units-stmt]
                                *must-stmt
                                [default-stmt]
                                [config-stmt]
                                [mandatory-stmt]
                                [status-stmt]
                                [description-stmt]
                                [reference-stmt]
                              "}" stmtsep */
    leaf_stmt: $ => Statement(alias('leaf', $.leaf_keyword), alias($._identifier_arg_str, $.leaf_arg_str),
      Block(repeat(choice(
        $.when_stmt,
        $.if_feature_stmt,
        $.type_stmt,
        $.units_stmt,
        $.must_stmt,
        $.default_stmt,
        $.config_stmt,
        $.mandatory_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt
      )))
    ),

    /** leaf-list-stmt      = leaf-list-keyword sep identifier-arg-str optsep
                         "{" stmtsep
                             ;; these stmts can appear in any order
                             [when-stmt]
                             *if-feature-stmt
                             type-stmt stmtsep
                             [units-stmt]
                             *must-stmt
                             *default-stmt
                             [config-stmt]
                             [min-elements-stmt]
                             [max-elements-stmt]
                             [ordered-by-stmt]
                             [status-stmt]
                             [description-stmt]
                             [reference-stmt]
                          "}" stmtsep
    */
    leaf_list_stmt: $ => Statement(alias('leaf-list', $.leaf_list_keyword), alias($._identifier_arg_str, $.leaf_list_arg_str),
      Block(repeat(choice(
        $.when_stmt,
        $.if_feature_stmt,
        $.type_stmt,
        $.units_stmt,
        $.must_stmt,
        $.default_stmt,
        $.config_stmt,
        $.min_elements_stmt,
        $.max_elements_stmt,
        $.ordered_by_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt
      )))
    ),

    /** list-stmt           = list-keyword sep identifier-arg-str optsep
                             "{" stmtsep
                                ;; these stmts can appear in any order
                                [when-stmt]
                                *if-feature-stmt
                                *must-stmt
                                [key-stmt]
                                *unique-stmt
                                [config-stmt]
                                [min-elements-stmt]
                                [max-elements-stmt]
                                [ordered-by-stmt]
                                [status-stmt]
                                [description-stmt]
                                [reference-stmt]
                                *(typedef-stmt / grouping-stmt)
                                1*data-def-stmt
                                *action-stmt
                                *notification-stmt
                              "}" stmtsep
    */
    list_stmt: $ => Statement(alias('list', $.list_keyword), alias($._identifier_arg_str, $.list_arg_str),
      Block(repeat(choice(
        $.when_stmt,
        $.if_feature_stmt,
        $.must_stmt,
        $.key_stmt,
        $.unique_stmt,
        $.config_stmt,
        $.min_elements_stmt,
        $.max_elements_stmt,
        $.ordered_by_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
        $.typedef_stmt,
        $.grouping_stmt,
        $._data_def_stmt, // repeat1?
        $.action_stmt,
        $.notification_stmt,
      )))
    ),

    /** key-stmt            = key-keyword sep key-arg-str stmtend
        key-arg-str         = < a string that matches the rule >
                              < key-arg >
        key-arg             = node-identifier *(sep node-identifier)
    */
    key_stmt: $ => NonBlockStmt(alias('key', $.key_keyword), $.key_arg_str),
    key_arg_str: $ => ArgStr($._key_arg),
    _key_arg: $ => seq($.node_identifier, repeat(seq($._sep, $.node_identifier))),

    /** unique-stmt         = unique-keyword sep unique-arg-str stmtend
        unique-arg-str      = < a string that matches the rule >
                              < unique-arg >
        unique-arg          = descendant-schema-nodeid
                              *(sep descendant-schema-nodeid)
    */
    unique_stmt: $ => NonBlockStmt(alias('unique', $.unique_keyword), $.unique_arg_str),
    unique_arg_str: $ => ArgStr($._unique_arg),
    _unique_arg: $ => seq($._descendant_schema_nodeid, repeat(seq($._sep, $._descendant_schema_nodeid))),

    _sep: _ => repeat1(choice(
      '\r\n',
      '\n',
      '\t',
      ' ',
    )),

    /** choice-stmt         = choice-keyword sep identifier-arg-str optsep
                             (";" /
                              "{" stmtsep
                                  ;; these stmts can appear in any order
                                  [when-stmt]
                                  *if-feature-stmt
                                  [default-stmt]
                                  [config-stmt]
                                  [mandatory-stmt]
                                  [status-stmt]
                                  [description-stmt]
                                  [reference-stmt]
                                  *(short-case-stmt / case-stmt)
                              "}") stmtsep */
    choice_stmt: $ => Statement(alias('choice', $.choice_keyword), alias($._identifier_arg_str, $.choice_arg_str),
      OptionalBlock(repeat(choice(
        $.when_stmt,
        $.if_feature_stmt,
        $.default_stmt,
        $.config_stmt,
        $.mandatory_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
        $._short_case_stmt,
        $.case_stmt,
      )))
    ),

    /** short-case-stmt     = choice-stmt /
                              container-stmt /
                              leaf-stmt /
                              leaf-list-stmt /
                              list-stmt /
                              anydata-stmt /
                              anyxml-stmt
    */
    _short_case_stmt: $ => choice(
      $.choice_stmt,
      $.container_stmt,
      $.leaf_stmt,
      $.leaf_list_stmt,
      $.list_stmt,
      $.anydata_stmt,
      $.anyxml_stmt,
    ),

    /** case-stmt           = case-keyword sep identifier-arg-str optsep
                             (";" /
                              "{" stmtsep
                                  ;; these stmts can appear in any order
                                  [when-stmt]
                                  *if-feature-stmt
                                  [status-stmt]
                                  [description-stmt]
                                  [reference-stmt]
                                  *data-def-stmt
                              "}") stmtsep */
    case_stmt: $ => Statement(alias('case', $.case_keyword), alias($._identifier_arg_str, $.case_arg_str),
      OptionalBlock(repeat(choice(
        $.when_stmt,
        $.if_feature_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
        $._data_def_stmt,
      )))
    ),

    /** anydata-stmt        = anydata-keyword sep identifier-arg-str optsep
                             (";" /
                              "{" stmtsep
                                  ;; these stmts can appear in any order
                                  [when-stmt]
                                  *if-feature-stmt
                                  *must-stmt
                                  [config-stmt]
                                  [mandatory-stmt]
                                  [status-stmt]
                                  [description-stmt]
                                  [reference-stmt]
                              "}") stmtsep */
    anydata_stmt: $ => Statement(alias('anydata', $.anydata_keyword), alias($._identifier_arg_str, $.anydata_arg_str),
      OptionalBlock(repeat(choice(
        $.when_stmt,
        $.if_feature_stmt,
        $.must_stmt,
        $.config_stmt,
        $.mandatory_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
      )))
    ),

    /** anyxml-stmt        = anyxml-keyword sep identifier-arg-str optsep
                           (";" /
                            "{" stmtsep
                                ;; these stmts can appear in any order
                                [when-stmt]
                                *if-feature-stmt
                                *must-stmt
                                [config-stmt]
                                [mandatory-stmt]
                                [status-stmt]
                                [description-stmt]
                                [reference-stmt]
                            "}") stmtsep */
    anyxml_stmt: $ => Statement(alias('anyxml', $.anyxml_keyword), alias($._identifier_arg_str, $.anyxml_arg_str),
      OptionalBlock(repeat(choice(
        $.when_stmt,
        $.if_feature_stmt,
        $.must_stmt,
        $.config_stmt,
        $.mandatory_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
      )))
    ),

    /** uses-stmt           = uses-keyword sep identifier-ref-arg-str optsep
                             (";" /
                              "{" stmtsep
                                  ;; these stmts can appear in any order
                                  [when-stmt]
                                  *if-feature-stmt
                                  [status-stmt]
                                  [description-stmt]
                                  [reference-stmt]
                                  *refine-stmt
                                  *uses-augment-stmt
                              "}") stmtsep
    */
    uses_stmt: $ => Statement(alias('uses', $.uses_keyword), alias(choice($._concatenated_string, $._identifier_ref_arg), $.uses_arg_str),
      OptionalBlock(repeat(choice(
        $.when_stmt,
        $.if_feature_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
        $.refine_stmt,
        $.uses_augment_stmt,
      )))
    ),

    /** refine-stmt         = refine-keyword sep refine-arg-str optsep
                              "{" stmtsep
                                  ;; these stmts can appear in any order
                                  *if-feature-stmt
                                  *must-stmt
                                  [presence-stmt]
                                  *default-stmt
                                  [config-stmt]
                                  [mandatory-stmt]
                                  [min-elements-stmt]
                                  [max-elements-stmt]
                                  [description-stmt]
                                  [reference-stmt]
                                "}" stmtsep
        refine-arg-str      = < a string that matches the rule >
                         < refine-arg >
        refine-arg          = descendant-schema-nodeid
    */
    refine_stmt: $ => Statement(alias('refine', $.refine_keyword), $.refine_arg_str,
      Block(repeat(choice(
        $.if_feature_stmt,
        $.must_stmt,
        $.presence_stmt,
        $.default_stmt,
        $.config_stmt,
        $.mandatory_stmt,
        $.min_elements_stmt,
        $.max_elements_stmt,
        $.description_stmt,
        $.reference_stmt,
      )))
    ),
    refine_arg_str: $ => QuotedOr($, $._refine_arg),
    _refine_arg: $ => $._descendant_schema_nodeid,

    /** uses-augment-stmt   = augment-keyword sep uses-augment-arg-str optsep
                             "{" stmtsep
                                ;; these stmts can appear in any order
                                [when-stmt]
                                *if-feature-stmt
                                [status-stmt]
                                [description-stmt]
                                [reference-stmt]
                                1*(data-def-stmt / case-stmt /
                                    action-stmt / notification-stmt)
                              "}" stmtsep
        uses-augment-arg-str = < a string that matches the rule >
                                < uses-augment-arg >
        uses-augment-arg     = descendant-schema-nodeid
   */
    uses_augment_stmt: $ => Statement(alias('augment', $.augment_keyword), $.uses_augment_arg_str,
      Block(repeat(choice(
        $.when_stmt,
        $.if_feature_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
        $._data_def_stmt,
        $.case_stmt,
        $.action_stmt,
        $.notification_stmt,
      )))
    ),
    uses_augment_arg_str: $ => prec.dynamic(0, QuotedOr($, $._descendant_schema_nodeid)),
    _uses_augment_arg: $ => $._descendant_schema_nodeid,

    /** augment-stmt        = augment-keyword sep augment-arg-str optsep
                             "{" stmtsep
                                ;; these stmts can appear in any order
                                [when-stmt]
                                *if-feature-stmt
                                [status-stmt]
                                [description-stmt]
                                [reference-stmt]
                                1*(data-def-stmt / case-stmt /
                                    action-stmt / notification-stmt)
                              "}" stmtsep

                              augment-arg-str     = < a string that matches the rule >
                                                    < augment-arg >

                              augment-arg         = absolute-schema-nodeid
    */
    augment_stmt: $ => Statement(alias('augment', $.augment_keyword), $.augment_arg_str,
      Block(repeat(choice(
        $.when_stmt,
        $.if_feature_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
        $._data_def_stmt,
        $.case_stmt,
        $.action_stmt,
        $.notification_stmt,
      )))
    ),
    augment_arg_str: $ => prec.dynamic(1, QuotedOr($, $._absolute_schema_nodeid)),
    _augment_arg: $ => $._absolute_schema_nodeid,

    /** rpc-stmt            = rpc-keyword sep identifier-arg-str optsep
                             (";" /
                              "{" stmtsep
                                  ;; these stmts can appear in any order
                                  *if-feature-stmt
                                  [status-stmt]
                                  [description-stmt]
                                  [reference-stmt]
                                  *(typedef-stmt / grouping-stmt)
                                  [input-stmt]
                                  [output-stmt]
                              "}") stmtsep */

    rpc_stmt: $ => Statement(alias('rpc', $.rpc_keyword), alias($._identifier_arg_str, $.rpc_arg_str),
      OptionalBlock(repeat(choice(
        $.if_feature_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
        $.typedef_stmt,
        $.grouping_stmt,
        $.input_stmt,
        $.output_stmt,
      )))
    ),

    /** action-stmt         = action-keyword sep identifier-arg-str optsep
                         (";" /
                          "{" stmtsep
                              ;; these stmts can appear in any order
                              *if-feature-stmt
                              [status-stmt]
                              [description-stmt]
                              [reference-stmt]
                              *(typedef-stmt / grouping-stmt)
                              [input-stmt]
                              [output-stmt]
                          "}") stmtsep
    */
    action_stmt: $ => Statement(alias('action', $.action_keyword), alias($._identifier_arg_str, $.action_arg_str),
      OptionalBlock(repeat(choice(
        $.if_feature_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
        $.typedef_stmt,
        $.grouping_stmt,
        $.input_stmt,
        $.output_stmt,
      )))
    ),

    /** input-stmt          = input-keyword optsep
                            "{" stmtsep
                                ;; these stmts can appear in any order
                                *must-stmt
                                *(typedef-stmt / grouping-stmt)
                                1*data-def-stmt
                            "}" stmtsep
        output-stmt         = output-keyword optsep
                            "{" stmtsep
                                ;; these stmts can appear in any order
                                *must-stmt
                                *(typedef-stmt / grouping-stmt)
                                1*data-def-stmt
                            "}" stmtsep */
    input_stmt: $ => NonArgStmt(alias('input', $.input_keyword),
      Block(repeat(choice(
        $.must_stmt,
        $.typedef_stmt,
        $.grouping_stmt,
        $._data_def_stmt, // repeat1?
      )))
    ),
    output_stmt: $ => NonArgStmt(alias('output', $.output_keyword),
      Block(repeat(choice(
        $.must_stmt,
        $.typedef_stmt,
        $.grouping_stmt,
        $._data_def_stmt, // repeat1?
      )))
    ),

    /** notification-stmt   = notification-keyword sep
                              identifier-arg-str optsep
                              (";" /
                              "{" stmtsep
                                  ;; these stmts can appear in any order
                                  *if-feature-stmt
                                  *must-stmt
                                  [status-stmt]
                                  [description-stmt]
                                  [reference-stmt]
                                  *(typedef-stmt / grouping-stmt)
                                  *data-def-stmt
                              "}") stmtsep */
    notification_stmt: $ => Statement(alias('notification', $.notification_keyword), alias($._identifier_arg_str, $.notification_arg_str),
      OptionalBlock(repeat(choice(
        $.if_feature_stmt,
        $.must_stmt,
        $.status_stmt,
        $.description_stmt,
        $.reference_stmt,
        $.typedef_stmt,
        $.grouping_stmt,
        $._data_def_stmt,
      )))
    ),

    /** deviation-stmt      = deviation-keyword sep
                              deviation-arg-str optsep
                              "{" stmtsep
                                  ;; these stmts can appear in any order
                                  [description-stmt]
                                  [reference-stmt]
                                  (deviate-not-supported-stmt /
                                    1*(deviate-add-stmt /
                                        deviate-replace-stmt /
                                        deviate-delete-stmt))
                              "}" stmtsep
        deviation-arg-str   = < a string that matches the rule >
                              < deviation-arg >
        deviation-arg       = absolute-schema-nodeid */
    deviation_stmt: $ => Statement(alias('deviation', $.deviation_keyword), $.deviation_arg_str,
      Block(repeat(choice(
        $.description_stmt,
        $.reference_stmt,
        $.deviate_not_supported_stmt,
        $.deviate_add_stmt,
        $.deviate_replace_stmt,
        $.deviate_delete_stmt,
      )))
    ),
    deviation_arg_str: $ => QuotedOr($, $._absolute_schema_nodeid),
    _deviation_arg: $ => $._absolute_schema_nodeid,

    /** deviate-not-supported-stmt =
                              deviate-keyword sep
                              not-supported-keyword-str stmtend
                              */
    deviate_not_supported_stmt: $ => NonBlockStmt(alias('deviate', $.deviate_keyword), $.not_supported_arg_str),
    not_supported_arg_str: _ => ArgStr(token(not_supported_arg)),

    /** deviate-add-stmt    = deviate-keyword sep add-keyword-str optsep
                              (";" /
                                "{" stmtsep
                                    ;; these stmts can appear in any order
                                    [units-stmt]
                                    *must-stmt
                                    *unique-stmt
                                    *default-stmt
                                    [config-stmt]
                                    [mandatory-stmt]
                                    [min-elements-stmt]
                                    [max-elements-stmt]
                                "}") stmtsep */
    deviate_add_stmt: $ => Statement(alias('deviate', $.deviate_keyword), $.add_arg_str,
      OptionalBlock(repeat(choice(
        $.units_stmt,
        $.must_stmt,
        $.unique_stmt,
        $.default_stmt,
        $.config_stmt,
        $.mandatory_stmt,
        $.min_elements_stmt,
        $.max_elements_stmt,
      )))
    ),
    add_arg_str: _ => ArgStr(token(add_arg)),

    /** deviate-delete-stmt = deviate-keyword sep delete-keyword-str optsep
                             (";" /
                               "{" stmtsep
                                   ;; these stmts can appear in any order
                                   [units-stmt]
                                   *must-stmt
                                   *unique-stmt
                                   *default-stmt
                               "}") stmtsep
   */
    deviate_delete_stmt: $ => Statement(alias('deviate', $.deviate_keyword), $.delete_arg_str,
      OptionalBlock(repeat(choice(
        $.units_stmt,
        $.must_stmt,
        $.unique_stmt,
        $.default_stmt,
      )))
    ),
    delete_arg_str: _ => ArgStr(token(delete_arg)),

    /** deviate-replace-stmt = deviate-keyword sep replace-keyword-str optsep
                               (";" /
                                "{" stmtsep
                                    ;; these stmts can appear in any order
                                    [type-stmt]
                                    [units-stmt]
                                    [default-stmt]
                                    [config-stmt]
                                    [mandatory-stmt]
                                    [min-elements-stmt]
                                    [max-elements-stmt]
                                "}") stmtsep */
    deviate_replace_stmt: $ => Statement(alias('deviate', $.deviate_keyword), $.replace_arg_str,
      OptionalBlock(repeat(choice(
        $.type_stmt,
        $.units_stmt,
        $.default_stmt,
        $.config_stmt,
        $.mandatory_stmt,
        $.min_elements_stmt,
        $.max_elements_stmt,
      )))
    ),
    replace_arg_str: _ => ArgStr(token(replace_arg)),

    /**
      ;; represents the usage of an extension
      unknown-statement   = prefix ":" identifier [sep string] optsep
                            (";" /
                              "{" optsep
                                  *((yang-stmt / unknown-statement) optsep)
                              "}") stmtsep
      The `{ ... }` body is parsed as an opaque, brace-balanced region (see
      `_brace_balanced`).
     */
    unknown_stmt: $ => seq(
      seq(alias($._prefix_arg, $.prefix), ':', $.identifier),
      optional(field('arg', choice($.string, $._slash_word, $._digit_start_word, $.integer_value, $._bare_word))),
      choice(
        ';',
        $._brace_balanced,
      ),
      stmtsep(),
    ),

    /** Matches a balanced `{ ... }` region without interpreting its content. */
    _brace_balanced: $ => seq(
      '{',
      repeat(choice($._brace_balanced, $._no_braces)),
      '}',
    ),
    _no_braces: _ => token(/[^{}]+/),

    _yang_stmt: $ => choice(
      $.action_stmt,
      $.anydata_stmt,
      $.anyxml_stmt,
      $.argument_stmt,
      $.augment_stmt,
      $.base_stmt,
      $.belongs_to_stmt,
      $.bit_stmt,
      $.case_stmt,
      $.choice_stmt,
      $.config_stmt,
      $.contact_stmt,
      $.container_stmt,
      $.default_stmt,
      $.description_stmt,
      $.deviate_add_stmt,
      $.deviate_delete_stmt,
      $.deviate_not_supported_stmt,
      $.deviate_replace_stmt,
      $.deviation_stmt,
      $.enum_stmt,
      $.error_app_tag_stmt,
      $.error_message_stmt,
      $.extension_stmt,
      $.feature_stmt,
      $.fraction_digits_stmt,
      $.grouping_stmt,
      $.identity_stmt,
      $.if_feature_stmt,
      $.import_stmt,
      $.include_stmt,
      $.input_stmt,
      $.key_stmt,
      $.leaf_list_stmt,
      $.leaf_stmt,
      $.length_stmt,
      $.list_stmt,
      $.mandatory_stmt,
      $.max_elements_stmt,
      $.min_elements_stmt,
      $.modifier_stmt,
      $.module_stmt,
      $.must_stmt,
      $.namespace_stmt,
      $.notification_stmt,
      $.ordered_by_stmt,
      $.organization_stmt,
      $.output_stmt,
      $.path_stmt,
      $.pattern_stmt,
      $.position_stmt,
      $.prefix_stmt,
      $.presence_stmt,
      $.range_stmt,
      $.reference_stmt,
      $.refine_stmt,
      $.require_instance_stmt,
      $.revision_date_stmt,
      $.revision_stmt,
      $.rpc_stmt,
      $.status_stmt,
      $.submodule_stmt,
      $.typedef_stmt,
      $.type_stmt,
      $.unique_stmt,
      $.units_stmt,
      $.uses_augment_stmt,
      $.uses_stmt,
      $.value_stmt,
      $.when_stmt,
      $.yang_version_stmt,
      $.yin_element_stmt,
    ),

    /** absolute-schema-nodeid = 1*("/" node-identifier)
        descendant-schema-nodeid =
                         node-identifier
                         [absolute-schema-nodeid]
    */
    _absolute_schema_nodeid: $ => repeat1(seq("/", $.node_identifier)),
    // RFC 7950: descendant-schema-nodeid = node-identifier
    //                                 [absolute-schema-nodeid]  (suffix optional)
    _descendant_schema_nodeid: $ => seq(
      $.node_identifier,
      optional($._absolute_schema_nodeid),
    ),

    /** when-stmt           = when-keyword sep string optsep
                             (";" /
                              "{" stmtsep
                                  ;; these stmts can appear in any order
                                  [description-stmt]
                                  [reference-stmt]
                              "}") stmtsep */
    when_stmt: $ => Statement(alias('when', $.when_keyword), $.string,
      OptionalBlock(repeat(choice(
        $.description_stmt,
        $.reference_stmt,
      )))
    ),

    /** config-stmt         = config-keyword sep
                         config-arg-str stmtend
        config-arg-str      = < a string that matches the rule >
                              < config-arg >
        config-arg          = true-keyword / false-keyword
    */
    config_stmt: $ => NonBlockStmt(alias('config', $.config_keyword), $._config_arg_str),
    _config_arg_str: $ => ArgStr($.boolean),

    /** mandatory-stmt      = mandatory-keyword sep
                         mandatory-arg-str stmtend
        mandatory-arg-str   = < a string that matches the rule >
                              < mandatory-arg >
        mandatory-arg       = true-keyword / false-keyword
   */
    mandatory_stmt: $ => NonBlockStmt(alias('mandatory', $.mandatory_keyword), $._mandatory_arg_str),
    _mandatory_arg_str: $ => ArgStr($.boolean),

    // presence-stmt       = presence-keyword sep string stmtend
    presence_stmt: $ => NonBlockStmt(alias('presence', $.presence_keyword), $.string),

    /** ordered-by-stmt     = ordered-by-keyword sep
                         ordered-by-arg-str stmtend
        ordered-by-arg-str  = < a string that matches the rule >
                              < ordered-by-arg >
        ordered-by-arg      = user-keyword / system-keyword
    */
    ordered_by_stmt: $ => NonBlockStmt(alias('ordered-by', $.ordered_by_keyword), $._ordered_by_arg_str),
    _ordered_by_arg_str: $ => ArgStr($._ordered_by_arg),
    _ordered_by_arg: _ => choice('user', 'system'),

    /** must-stmt           = must-keyword sep string optsep
                         (";" /
                          "{" stmtsep
                              ;; these stmts can appear in any order
                              [error-message-stmt]
                              [error-app-tag-stmt]
                              [description-stmt]
                              [reference-stmt]
                           "}") stmtsep
    */
    must_stmt: $ => Statement(alias('must', $.must_keyword), alias($.string, $.must_expression),
      OptionalBlock(repeat(choice(
        $.error_message_stmt,
        $.error_app_tag_stmt,
        $.description_stmt,
        $.reference_stmt
      ))),
    ),

    /** min-elements-stmt   = min-elements-keyword sep
                               min-value-arg-str stmtend
        min-value-arg-str   = < a string that matches the rule >
                              < min-value-arg >
        min-value-arg       = non-negative-integer-value
        max-elements-stmt   = max-elements-keyword sep
                              max-value-arg-str stmtend
        max-value-arg-str   = < a string that matches the rule >
                              < max-value-arg >
        max-value-arg       = unbounded-keyword /
                              positive-integer-value
    */
    min_elements_stmt: $ => NonBlockStmt(alias('min-elements', $.min_elements_keyword), $._minvalue_arg_str),
    _minvalue_arg_str: $ => ArgStr($._min_value_arg),
    _min_value_arg: $ => $._non_negative_integer_value,
    max_elements_stmt: $ => NonBlockStmt(alias('max-elements', $.max_elements_keyword), $._maxvalue_arg_str),
    _maxvalue_arg_str: $ => ArgStr($._max_value_arg),
    _max_value_arg: $ => choice('unbounded', $._positive_integer_value),

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
    _non_zero_digit: _ => token(/[1-9]/),
    _DIGIT: _ => token(/[0-9]/),

    /** zero-integer-value  = 1*DIGIT
        decimal-value       = integer-value ("." zero-integer-value) */
    _zero_integer_value: $ => repeat1($._DIGIT),
    decimal_value: $ => seq(
      alias($.integer_value, "integer_part"),
      optional(seq('.', $._zero_integer_value))
    ),

    // Copied from "tree-sitter-javascript":
    // https://github.com/tree-sitter/tree-sitter-javascript/blob/2c5b138ea488259dbf11a34595042eb261965259/grammar.js#L907
    comment: $ => token(choice(
      seq('//', /.*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
        '/'
      )
    )),

    prefix_arg_str: $ => ArgStr($._prefix_arg),
    _prefix_arg: $ => $.identifier,

    _identifier_ref_arg_str: $ => ArgStr($._identifier_ref_arg),
    _identifier_ref_arg: $ => seq(
      optional(seq(
        alias($._prefix_arg, $.prefix),
        ':')),
      $.identifier),

    // Bare (unquoted) word starting with a digit (e.g. an enum name).
    _digit_start_word: _ => token(/[0-9][0-9A-Za-z\-_.]*/),
    // Bare (unquoted) word containing '/' (e.g. units like Mb/s).
    _slash_word: _ => token(/[^"';\s{}]*\/[^"';\s{}]*/),
    // Bare (unquoted) word arguments that contain at least one character
    // outside the identifier set (letters/digits/_/./-) — RFC 7950
    // unquoted-string / enum names / identityref or time defaults with `:`,
    // e.g. `units m^-X`, `enum n+1`, `default 00:00:15.0`,
    // `default syslogtypes:local7`. The required symbol class also excludes
    // quotes, `;`, whitespace and braces, so the token never competes with
    // `identifier`, `_slash_word` or `_digit_start_word` on ordinary names.
    _bare_word: _ => token(/[^"';\s{}]*[^A-Za-z0-9_.\s"';{}-][^"';\s{}]*/),
    // Junk-word fallback for the `type` argument only: like `_bare_word` but
    // the required symbol excludes ':' (and '-'), so a prefixed type such as
    // `iana:if-type` still parses as prefix + identifier instead of being
    // swallowed whole; genuine typos/symbols (e.g. `type ^bad`) stay local.
    _type_junk_word: _ => token(/[^"';\s{}]*[^A-Za-z0-9_.\s"';{}:-][^"';\s{}]*/),

    _identifier_arg_str: $ => ArgStr($._identifier_arg),
    _identifier_arg: $ => $.identifier,
    identifier: _ => {
      const alpha_underscore = /[a-zA-Z_]/
      const alphanumeric = /[a-zA-Z0-9-_.]/
      return token(seq(alpha_underscore, repeat(alphanumeric)))
    },

    /**
     * @see {@link https://www.rfc-editor.org/rfc/rfc7950#section-6.1.3 Quoting}
     */
    // YANG single-quoted strings (RFC 7950 §6.1.3) perform no escape
    // processing, so a backslash is an ordinary character inside them.
    _unescaped_string1: _ => token.immediate(prec(1, /[^']+/)),
    // unescaped text inside double quotes (a backslash starts an escape)
    _unescaped_string2: _ => token.immediate(prec(1, /[^"\\]+/)),
    // escape sequence inside double-quoted strings. RFC 7950 defines \n \t
    // \" \\; real modules also carry other escapes (e.g. `\*`, `\S`, `\.`
    // inside `pattern`), which pyang tolerates (it only warns) — keep them
    // parseable and let semantic layers judge strictness.
    _escape_sequence: _ => token.immediate(/\\[\s\S]/),

    _single_quoted_string: $ => SingleQuoted(
      repeat1($._unescaped_string1)),

    _double_quoted_string: $ => DoubleQuoted(
      repeat1(choice(
        $._unescaped_string2,
        $._escape_sequence,
      ))),

    _empty_string: _ => choice('""', "''"),

    quoted_string: $ => choice(
      $._single_quoted_string,
      $._double_quoted_string,
      $._empty_string
    ),

    _concatenated_string: $ => PlusSep1($.quoted_string),

    // Bare (unquoted) URI argument for namespace-stmt, lexed up to the
    // statement terminator (':' and other URI characters are allowed).
    _uri_str: _ => token(/[^"';\s{}]+/),

    string: $ => choice($._concatenated_string, $.identifier),

    boolean: _ => choice('true', 'false'),
  }
})

function LF() {
  return token('\n')
}

function CRLF() {
  return token('\r\n')
}

function SP() {
  return token(' ')
}

function HTAP() {
  return token('\t')
}

function WSP() {
  return choice(SP(), HTAP())
}

function line_break() {
  return choice(CRLF(), LF())
}

function sep() {
  return repeat1(choice(line_break(), WSP()))
}

/**
 * Creates a rule for
 * stmtsep             = *(WSP / line-break / unknown-statement)
 * @returns {RepeatRule}
 */
function stmtsep() {
  return repeat(choice(WSP(), line_break(), sym('unknown_stmt')))
}

/**
 * Creates a rule to match one or more of the rules separated by a bar
 *
 * @param {Rule} rule
 *
 * @returns {SeqRule}
 */
function BarSep1(rule) {
  return seq(rule, repeat(seq('|', rule)))
}

/**
 * Creates a rule to match one or more of the rules separated by a plus
 *
 * @param {Rule} rule
 *
 * @returns {SeqRule}
 */
function PlusSep1(rule) {
  return seq(rule, repeat(seq('+', rule)))
}

/**
 * Creates a single-quoted rule
 *
 * @param {Rule} rule YANG rule
 * @returns {Rule}
 */
function SingleQuoted(rule) {
  return seq("'", rule, "'")
}

/**
 * Creates a double-quoted rule
 *
 * @param {Rule} rule YANG rule
 * @returns {Rule}
 */
function DoubleQuoted(rule) {
  return seq('"', rule, '"')
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
  )
}

/**
 * An argument whose value is "a string that matches <content>".
 *
 * Quoted arguments (single or '+' concatenated pieces) are kept as opaque
 * strings; only unquoted (bare) arguments are parsed as structured <content>.
 *
 * @param {object} $ grammar rule proxy
 * @param {Rule} content
 * @returns {Rule}
 */
function QuotedOr($, content) {
  return choice($._concatenated_string, content)
}

/**
 * Creates a YANG statement with a block of sub-statements.
 *
 * @param {Rule} rule YANG rule
 * @returns {Rule}
 */
function Block(rule) {
  return seq(
    '{', stmtsep(), rule,
    '}')
}

/**
 * Creates a YANG statement with 0-1 block of sub-statements.
 *
 * @param {Rule} rule YANG rule
 * @returns {Rule}
 */
function OptionalBlock(rule) {
  return choice(
    ';',
    Block(rule)
  )
}

/**
 * Creates a YANG statement with no argument field. E.g., "input" | "output"
 *
 * @param {string} keyword YANG keyword
 * @param {Rule} block substatement block of the statement
 * @returns {Rule} YANG statement
 */
function NonArgStmt(keyword, block) {
  return seq(keyword, block, stmtsep())
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
    choice(
      seq(';', stmtsep()),
      seq('{', stmtsep(), '}', stmtsep())
    ),
  )
}

/**
 * Creates a YANG statement
 *
 * @param {string} keyword YANG keyword
 * @param {Rule} argument argument of the keyword
 * @param {Rule} block substatement block of the statement
 * @param {boolean} tail_stmtsep if the statement allows a tail stmtsep
 * @returns {Rule} YANG statement
 */
function Statement(keyword, argument, block, tail_stmtsep = true) {
  if (!argument) {
    return NonArgStmt(keyword, block)
  }
  if (!block) {
    return NonBlockStmt(keyword, argument)
  }
  if (tail_stmtsep) {
    return seq(keyword, field('arg', argument), block, stmtsep())
  }
  else {
    return seq(keyword, field('arg', argument), block)
  }
}
