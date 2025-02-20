import XCTest
import SwiftTreeSitter
import TreeSitterAdventure

final class TreeSitterAdventureTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_adventure())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Adventure grammar")
    }
}
