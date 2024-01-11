const std = @import("std");

noinline fn foo(x: u32) u32 {
    return x * x;
}


noinline fn bar() u32{
    return foo(std.math.maxInt(u32));
}

pub fn main() !void {
    std.debug.print("{}", .{ bar()});
}
