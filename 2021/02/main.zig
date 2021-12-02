const print = @import("std").debug.print;
const std = @import("std");

pub fn main() !void {
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    var x: i32 = 0;
    var y: i32 = 0;

    var x_2: i32 = 0;
    var y_2: i32 = 0;
    var aim: i32 = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var tokens = std.mem.tokenize(u8, line, " ");
        var command = tokens.next().?;
        var value = try std.fmt.parseInt(i32, tokens.next().?, 10);

        if (std.mem.eql(u8, command, "forward")) {
            x += value;

            x_2 += value;
            y_2 += aim * value;
        } else if (std.mem.eql(u8, command, "up")) {
            y -= value;
            aim -= value;
        } else if (std.mem.eql(u8, command, "down")) {
            y += value;
            aim += value;
        }
    }
    print("{}\n{}\n", .{x * y, x_2 * y_2});
}
