import XCTest
@testable import dependencies

final class dependenciesTests: XCTestCase {
    func testExample() {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct
        // results.
        XCTAssertEqual(dependencies().text, "Hello, World!")
    }

    static var allTests = [
        ("testExample", testExample),
    ]
}
