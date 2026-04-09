const std = @import("std");

pub fn build(b: *std.Build) void {
    const debug = b.dependency("zigci", .{ .release = false });
    const release = b.dependency("zigci", .{ .release = false });

    const ci = b.step("ci", "Run CI");

    const test_results = b.addWriteFiles();
    _ = test_results.addCopyFile(debug.namedLazyPath("evidence"), "debug");
    _ = test_results.addCopyFile(release.namedLazyPath("evidence"), "release");
    ci.dependOn(&test_results.step);
}
