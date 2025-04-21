import XCTest
import SwiftTreeSitter
import TreeSitterLatex

final class TreeSitterLatexTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_latex())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Latex grammar")
    }
}
