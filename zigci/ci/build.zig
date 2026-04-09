const std = @import("std");

pub fn build(b: *std.Build) void {
    const debug = b.dependency("zigci", .{ .release = false });
    const release = b.dependency("zigci", .{ .release = true });

    const ci = b.step("ci", "Run CI");

    ci.dependOn(&debug.builder.top_level_steps.get("test").?.step);
    ci.dependOn(&release.builder.top_level_steps.get("test").?.step);
}
