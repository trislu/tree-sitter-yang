package tree_sitter_yang_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_yang "github.com/trislu/tree-sitter-yang/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_yang.Language())
	if language == nil {
		t.Errorf("Error loading Yang grammar")
	}
}
