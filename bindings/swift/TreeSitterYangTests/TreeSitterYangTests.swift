import XCTest
import SwiftTreeSitter
import TreeSitterYang

final class TreeSitterYangTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_yang())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Yang grammar")
    }
}
