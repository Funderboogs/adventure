package tree_sitter_adventure_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_adventure "github.com/unquabain/adventure/tree-sitter-adventure/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_adventure.Language())
	if language == nil {
		t.Errorf("Error loading Adventure grammar")
	}
}
