import XCTest
import SwiftTreeSitter
import TreeSitterClojure

final class TreeSitterClojureTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_clojure())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Clojure grammar")
    }
}
