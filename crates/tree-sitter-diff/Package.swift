// swift-tools-version:5.3

import Foundation
import PackageDescription

var sources = ["src/parser.c"]
if FileManager.default.fileExists(atPath: "src/scanner.c") {
    sources.append("src/scanner.c")
}

let package = Package(
    name: "TreeSitterDiff",
    products: [
        .library(name: "TreeSitterDiff", targets: ["TreeSitterDiff"]),
    ],
    dependencies: [
        .package(url: "https://github.com/tree-sitter/swift-tree-sitter", from: "0.8.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterDiff",
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
            name: "TreeSitterDiffTests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterDiff",
            ],
            path: "bindings/swift/TreeSitterDiffTests"
        )
    ],
    cLanguageStandard: .c11
)
