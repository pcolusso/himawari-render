import XCTest

import dependenciesTests

var tests = [XCTestCaseEntry]()
tests += dependenciesTests.allTests()
XCTMain(tests)
