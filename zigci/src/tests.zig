const std = @import("std");
const builtin = @import("builtin");

test "smoke" {
    switch (builtin.mode) {
        .Debug => return error.FailInDebug,
        else => {},
    }
}
