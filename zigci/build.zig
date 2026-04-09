const std = @import("std");

pub fn build(b: *std.Build) void {
    const optimize = b.standardOptimizeOption(.{ .preferred_optimize_mode = .ReleaseSafe });
    const t = b.addTest(.{
        .root_module = b.addModule("test", .{
            .root_source_file = b.path("src/tests.zig"),
            .target = b.graph.host,
            .optimize = optimize,
        }),
    });
    const t_run = b.addRunArtifact(t);
    b.step("test", "Run tests").dependOn(&t_run.step);
}
