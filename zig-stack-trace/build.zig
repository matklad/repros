const std = @import("std");

pub fn build(b: *std.Build) !void {
    const exe = b.addExecutable(.{
        .name = "hoyten",
        .root_source_file = b.path("./main.zig"),
        .target = b.standardTargetOptions(.{}),
        .optimize = .ReleaseSafe,
    });
    exe.root_module.omit_frame_pointer = false;

    switch (b.option(enum { strip, objcopy, llvm_objcopy, none }, "strip", "") orelse .none) {
        .none => b.installArtifact(exe),
        .strip => {
            exe.root_module.strip = true;
            b.installArtifact(exe);
        },
        .objcopy => {
            const stripped_exe = b.addObjCopy(exe.getEmittedBin(), .{
                .basename = exe.out_filename, // set the name for the debuglink
                .strip = .debug,
            });
            b.getInstallStep().dependOn(&b.addInstallBinFile(
                stripped_exe.getOutput(),
                exe.out_filename,
            ).step);
        },
        .llvm_objcopy => {
            const objcopy = b.addSystemCommand(&.{ "llvm-objcopy", "--strip-debug" });
            objcopy.addArtifactArg(exe);
            b.getInstallStep().dependOn(&b.addInstallBinFile(
                objcopy.addOutputFileArg(exe.out_filename),
                exe.out_filename,
            ).step);
        },
    }
}
