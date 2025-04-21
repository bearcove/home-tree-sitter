// swift-tools-version:5.3

import Foundation
import PackageDescription

var sources = ["src/parser.c"]
if FileManager.default.fileExists(atPath: "src/scanner.c") {
    sources.append("src/scanner.c")
}

let package = Package(
    name: "TreeSitterJinja2",
    products: [
        .library(name: "TreeSitterJinja2", targets: ["TreeSitterJinja2"]),
    ],
    dependencies: [
        .package(url: "https://github.com/tree-sitter/swift-tree-sitter", from: "0.8.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterJinja2",
            dependencies: [],
            path: ".",
            sources: sources,
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift",
            cSettings: [.headerSearchPath("src")]
        ),
        .testTarget(
            name: "TreeSitterJinja2Tests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterJinja2",
            ],
            path: "bindings/swift/TreeSitterJinja2Tests"
        )
    ],
    cLanguageStandard: .c11
)
