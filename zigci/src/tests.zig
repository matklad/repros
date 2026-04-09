const std = @import("std");
const builtin = @import("builtin");

test "smoke" {
    if (true) return;
    switch (builtin.mode) {
        .Debug => return error.FailInDebug,
        else => {},
    }
}
