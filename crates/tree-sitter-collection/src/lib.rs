use tree_sitter::QueryError;
use tree_sitter_highlight::HighlightConfiguration;

pub use tree_sitter;
pub use tree_sitter_highlight;

pub fn go() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_go::LANGUAGE.into(),
        "go",
        tree_sitter_go::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn c() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_c::LANGUAGE.into(),
        "c",
        tree_sitter_c::HIGHLIGHT_QUERY,
        "",
        "",
    )
}

pub fn cpp() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_cpp::LANGUAGE.into(),
        "cpp",
        &[
            tree_sitter_c::HIGHLIGHT_QUERY,
            tree_sitter_cpp::HIGHLIGHT_QUERY,
        ]
        .join("\n"),
        "",
        "",
    )
}

pub fn rust() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_rust::LANGUAGE.into(),
        "rust",
        tree_sitter_rust::HIGHLIGHTS_QUERY,
        tree_sitter_rust::INJECTIONS_QUERY,
        "",
    )
}

pub fn javascript() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_javascript::LANGUAGE.into(),
        "javascript",
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_javascript::INJECTIONS_QUERY,
        tree_sitter_javascript::LOCALS_QUERY,
    )
}

pub fn java() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_java::LANGUAGE.into(),
        "java",
        tree_sitter_java::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn typescript() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "typescript",
        r#"
        ; Types

        (type_identifier) @type
        (predefined_type) @type.builtin

        ((identifier) @type
         (#match? @type "^[A-Z]"))

        (type_arguments
          "<" @punctuation.bracket
          ">" @punctuation.bracket)

        ; Variables

        (required_parameter (identifier) @variable.parameter)
        (optional_parameter (identifier) @variable.parameter)

        ; Keywords

        [ "abstract"
          "declare"
          "enum"
          "export"
          "implements"
          "interface"
          "keyof"
          "namespace"
          "private"
          "protected"
          "public"
          "type"
          "readonly"
          "override"
          "satisfies"
          "import"
          "from"
        ] @keyword

        (string) @string
        (identifier) @parameter
        "#,
        "",
        tree_sitter_typescript::LOCALS_QUERY,
    )
}

pub fn tsx() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_typescript::LANGUAGE_TSX.into(),
        "tsx",
        tree_sitter_typescript::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn toml() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_toml::language(),
        "toml",
        tree_sitter_toml::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn bash() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_bash::LANGUAGE.into(),
        "bash",
        tree_sitter_bash::HIGHLIGHT_QUERY,
        "",
        "",
    )
}

pub fn html() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_html::LANGUAGE.into(),
        "html",
        tree_sitter_html::HIGHLIGHTS_QUERY,
        tree_sitter_html::INJECTIONS_QUERY,
        "",
    )
}

pub fn python() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_python::LANGUAGE.into(),
        "python",
        tree_sitter_python::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn ini() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_ini::language(),
        "ini",
        tree_sitter_ini::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn meson() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_meson::language(),
        "meson",
        tree_sitter_meson::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn x86asm() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_x86asm::language(),
        "x86asm",
        tree_sitter_x86asm::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn asm() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_asm::LANGUAGE.into(),
        "asm",
        tree_sitter_asm::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn yaml() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_yaml::LANGUAGE.into(),
        "yaml",
        tree_sitter_yaml::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn dockerfile() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_dockerfile::language(),
        "dockerfile",
        tree_sitter_dockerfile::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn nix() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_nix::language(),
        "nix",
        tree_sitter_nix::HIGHLIGHTS_QUERY,
        tree_sitter_nix::INJECTIONS_QUERY,
        tree_sitter_nix::LOCALS_QUERY,
    )
}

pub fn clojure() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_clojure::LANGUAGE.into(),
        "clojure",
        tree_sitter_clojure::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn zig() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_zig::language(),
        "zig",
        tree_sitter_zig::HIGHLIGHTS_QUERY,
        tree_sitter_zig::INJECTIONS_QUERY,
        "",
    )
}

pub fn diff() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_diff::LANGUAGE.into(),
        "diff",
        r#"
[(addition) (new_file)] @diff.addition
[(deletion) (old_file)] @diff.deletion

(commit) @constant
(location) @attribute
(command) @variable.builtin
        "#,
        "",
        "",
    )
}

pub fn css() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_css::LANGUAGE.into(),
        "css",
        tree_sitter_css::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

pub fn jinja() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_jinja2::LANGUAGE.into(),
        "jinja",
        "(expression) @string
        (statement) @variable.builtin
        (keyword) @keyword
        (comment) @comment
        (identifier) @parameter
        (operator) @operator
        (string) @string",
        "",
        "",
    )
}

pub fn markdown() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_md::LANGUAGE.into(),
        "markdown",
        tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
        r#"
        (fenced_code_block
          (info_string
            (language) @injection.language)
          (code_fence_content) @injection.content)

        ((html_block) @injection.content (#set! injection.language "html") (#set! injection.include-children))

        (document . (section . (thematic_break) (_) @injection.content (thematic_break)) (#set! injection.language "yaml") (#set! injection.include-children))

        ([(minus_metadata) (plus_metadata)] @injection.content (#set! injection.language "yml") (#set! injection.include-children))

        ((inline) @injection.content (#set! injection.language "markdown_inline") (#set! injection.include-children))
        "#,
        "",
    )
}

pub fn markdown_inline() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_md::INLINE_LANGUAGE.into(),
        "markdown_inline",
        r#"
        ;; From nvim-treesitter/nvim-treesitter
        [
          (code_span)
          (link_title)
        ] @text.literal

        [
          (emphasis_delimiter)
          (code_span_delimiter)
        ] @punctuation.delimiter

        (emphasis) @text.emphasis

        (strong_emphasis) @text.strong

        (strikethrough) @text.strikethrough

        [
          (link_destination)
          (uri_autolink)
        ] @text.uri

        [
          (link_label)
          (link_text)
          (image_description)
        ] @text.reference

        [
          (backslash_escape)
          (hard_line_break)
        ] @string.escape

        (image ["!" "[" "]" "(" ")"] @punctuation.delimiter)
        (inline_link ["[" "]" "(" ")"] @punctuation.delimiter)
        (shortcut_link ["[" "]"] @punctuation.delimiter)

        ; NOTE: extension not enabled by default
        ; (wiki_link ["[" "|" "]"] @punctuation.delimiter)
        "#,
        r#"
        ((html_tag) @injection.content (#set! injection.language "html") (#set! injection.include-children))
        ((latex_block) @injection.content (#set! injection.language "latex") (#set! injection.include-children))
        "#,
        "",
    )
}

pub fn latex() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_latex::LANGUAGE.into(),
        "latex",
        r#"
        (operator) @operator
        (word) @keyword
        "#,
        "",
        "",
    )
}

pub fn scss() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        tree_sitter_scss::LANGUAGE.into(),
        "scss",
        tree_sitter_scss::HIGHLIGHTS_QUERY,
        "",
        "",
    )
}

#[cfg(test)]
mod tests {
    use tree_sitter_highlight::Highlighter;

    use super::*;

    #[test]
    fn build_go() {
        go().unwrap();
    }

    #[test]
    fn build_c() {
        c().unwrap();
    }

    #[test]
    fn build_cpp() {
        cpp().unwrap();
    }

    #[test]
    fn build_rust() {
        let recognized_names = ["keyword".to_string()];
        let mut conf = rust().unwrap();
        conf.configure(&recognized_names[..]);

        let mut highlighter = Highlighter::new();
        let source = b"async fn main() {}";

        let highlights = highlighter
            .highlight(&conf, source, None, |_| None)
            .unwrap();
        for highlight in highlights {
            let highlight = highlight.unwrap();
            println!("highlight event: {highlight:#?}");
        }
    }

    #[test]
    fn build_javascript() {
        javascript().unwrap();
    }

    #[test]
    fn build_java() {
        java().unwrap();
    }

    #[test]
    fn build_typescript() {
        typescript().unwrap();
    }

    #[test]
    fn build_tsx() {
        tsx().unwrap();
    }

    #[test]
    fn build_toml() {
        toml().unwrap();
    }

    #[test]
    fn build_bash() {
        bash().unwrap();
    }

    #[test]
    fn build_html() {
        html().unwrap();
    }

    #[test]
    fn build_python() {
        python().unwrap();
    }

    #[test]
    fn build_ini() {
        ini().unwrap();
    }

    #[test]
    fn build_meson() {
        meson().unwrap();
    }

    #[test]
    fn build_x86asm() {
        x86asm().unwrap();
    }

    #[test]
    fn build_asm() {
        asm().unwrap();
    }

    #[test]
    fn build_yaml() {
        yaml().unwrap();
    }

    #[test]
    fn build_dockerfile() {
        dockerfile().unwrap();
    }

    #[test]
    fn build_nix() {
        nix().unwrap();
    }

    #[test]
    fn build_clojure() {
        clojure().unwrap();
    }

    #[test]
    fn build_zig() {
        zig().unwrap();
    }

    #[test]
    fn build_diff() {
        diff().unwrap();
    }

    #[test]
    fn build_css() {
        css().unwrap();
    }

    #[test]
    fn build_jinja() {
        jinja().unwrap();
    }

    #[test]
    fn build_markdown() {
        markdown().unwrap();
    }

    #[test]
    fn build_markdown_inline() {
        markdown_inline().unwrap();
    }

    #[test]
    fn build_latex() {
        latex().unwrap();
    }

    #[test]
    fn build_scss() {
        scss().unwrap();
    }
}
