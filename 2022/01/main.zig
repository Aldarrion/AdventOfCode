const print = @import("std").debug.print;
const std = @import("std");

pub fn main() !void {
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    // Result of `try ...` is an optional. |variable_name| syntax unwraps the optional to a variable when valid
    // otherwise it ends the while loop
    var current_elf: i32 = 0;
    var max_elf = std.mem.zeroes([3]i32);
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line_raw| {
        var line = std.mem.trim(u8, line_raw, &std.ascii.spaces);
        if (line.len == 0) {
            for (max_elf) |*elf| {
                if (current_elf > elf.*) {
                    var tmp = elf.*;
                    elf.* = current_elf;
                    current_elf = tmp;
                }
            }
            current_elf = 0;
        } else {
            var calories = try std.fmt.parseInt(i32, line, 10);
            current_elf += calories;
        }
    }

    for (max_elf) |*elf| {
        if (current_elf > elf.*) {
            var tmp = elf.*;
            elf.* = current_elf;
            current_elf = tmp;
        }
    }

    var sum: i32 = 0;
    var max: i32 = 0;
    for (max_elf) |elf| {
        if (elf > max) {
            max = elf;
        }
        sum += elf;
    }

    print("{}\n{}\n", .{ max, sum });
}
