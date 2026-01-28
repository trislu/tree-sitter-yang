/**
 * @file Yang grammar for tree-sitter
 * @author trislu <lukai@mail.ustc.edu.cn>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

export default grammar({
  name: "yang",

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => "hello"
  }
});
