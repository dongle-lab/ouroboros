const std = @import("std");
const expect = std.testing.expect;

fn fibo(n: u16) u16 {
    if (n == 0 or n == 1) return n;
    return fibo(n - 1) + fibo(n - 2);
}

test "recursive" {
    const x = fibo(10);
    try expect(x == 55);
}

pub fn main() void {
    std.debug.print("Hello, {s}!\n", .{"World"});
}
