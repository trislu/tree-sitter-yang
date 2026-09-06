; Syntax highlighting captures for YANG (RFC 7950), driven by this grammar's
; CST shape: every statement is a `<name>_stmt` node whose keyword is a
; `<name>_keyword` node and whose argument is a `<name>_arg_str` node wrapping
; leaf tokens (`identifier`, `quoted_string`, `date_str`, `integer_value`,
; `decimal_value`, …).
;
; Standard captures only (@keyword/@string/@comment/@number/@boolean/@type/
; @variable/@namespace) so any consumer theme (Zed, Neovim/nvim-treesitter,
; tree-sitter CLI) colors YANG out of the box. The class choices mirror the
; language server's semantic-token classification (namespace-ish names,
; type/grouping/identity references, data-node names, dates as strings).

; ---------------------------------------------------------------------------
; Keywords — every statement keyword (enumeration derived from node-types).
; ---------------------------------------------------------------------------
[(action_keyword) (anydata_keyword) (anyxml_keyword) (argument_keyword) (augment_keyword) (base_keyword)] @keyword
[(belongs_to_keyword) (bit_keyword) (case_keyword) (choice_keyword) (config_keyword) (contact_keyword)] @keyword
[(container_keyword) (default_keyword) (description_keyword) (deviate_keyword) (deviation_keyword) (enum_keyword)] @keyword
[(error_app_tag_keyword) (error_message_keyword) (extension_keyword) (feature_keyword) (fraction_digits_keyword) (grouping_keyword)] @keyword
[(identity_keyword) (if_feature_keyword) (import_keyword) (include_keyword) (input_keyword) (key_keyword)] @keyword
[(leaf_keyword) (leaf_list_keyword) (length_keyword) (list_keyword) (mandatory_keyword) (max_elements_keyword)] @keyword
[(min_elements_keyword) (modifier_keyword) (module_keyword) (must_keyword) (namespace_keyword) (notification_keyword)] @keyword
[(ordered_by_keyword) (organization_keyword) (output_keyword) (path_keyword) (pattern_keyword) (position_keyword)] @keyword
[(prefix_keyword) (presence_keyword) (range_keyword) (reference_keyword) (refine_keyword) (require_instance_keyword)] @keyword
[(revision_date_keyword) (revision_keyword) (rpc_keyword) (status_keyword) (submodule_keyword) (type_keyword)] @keyword
[(typedef_keyword) (unique_keyword) (units_keyword) (uses_keyword) (value_keyword) (when_keyword)] @keyword
[(yang_version_keyword) (yin_element_keyword)] @keyword

; Sub-statement "verb" words that are anonymous tokens (not identifier nodes):
; `deviate` variants and the `modifier` / `status` values.
["add" "delete" "replace" "not-supported"] @keyword
["invert-match"] @keyword
["current" "deprecated" "obsolete"] @keyword

; ---------------------------------------------------------------------------
; Comments and literals.
; ---------------------------------------------------------------------------
(comment) @comment

(quoted_string) @string
(date_str) @string

; Bare numbers (`default 7`, `range "1..10"`, `fraction-digits 2`) are folded
; into their argument span by the grammar (no leaf node survives, see the
; token-stream notes in yrepo), so they cannot be captured here; `true`/`false`
; survive as anonymous tokens.
["true" "false"] @boolean

; ---------------------------------------------------------------------------
; Module / submodule / prefix names (namespace-ish).
; ---------------------------------------------------------------------------
(module_arg_str (identifier) @namespace)
(submodule_arg_str (identifier) @namespace)
(import_arg_str (identifier) @namespace)
(include_arg_str (identifier) @namespace)
(belongs_to_arg_str (identifier) @namespace)
(prefix_arg_str (identifier) @namespace)
; The prefix half of a `prefix:name` reference.
(prefix (identifier) @namespace)

; ---------------------------------------------------------------------------
; Type / grouping / identity references.
; ---------------------------------------------------------------------------
(type_arg_str (identifier) @type)
(base_arg_str (identifier) @type)
(uses_arg_str (identifier) @type)

; ---------------------------------------------------------------------------
; Data-node names and definition names.
; ---------------------------------------------------------------------------
(container_arg_str (identifier) @variable)
(leaf_arg_str (identifier) @variable)
(leaf_list_arg_str (identifier) @variable)
(list_arg_str (identifier) @variable)
(choice_arg_str (identifier) @variable)
(case_arg_str (identifier) @variable)
(anyxml_arg_str (identifier) @variable)
(anydata_arg_str (identifier) @variable)
(rpc_arg_str (identifier) @variable)
(action_arg_str (identifier) @variable)
(notification_arg_str (identifier) @variable)
(grouping_arg_str (identifier) @variable)
(typedef_arg_str (identifier) @variable)
(identity_arg_str (identifier) @variable)
(feature_arg_str (identifier) @variable)
(extension_arg_str (identifier) @variable)
(bit_arg_str (identifier) @variable)
; `enum` names are `string`s (quoted or bare identifier); a bare symbol or
; digit-starting word has no node of its own and stays uncolored.
(enum_arg_str (string (identifier) @variable))
