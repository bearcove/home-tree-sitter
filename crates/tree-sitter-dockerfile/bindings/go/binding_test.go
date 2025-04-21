package tree_sitter_dockerfile_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-dockerfile"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_dockerfile.Language())
	if language == nil {
		t.Errorf("Error loading Dockerfile grammar")
	}
}
