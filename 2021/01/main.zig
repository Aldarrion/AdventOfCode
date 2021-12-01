const print = @import("std").debug.print;
const std = @import("std");

pub fn main() !void {
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    var inc_count_1: i32 = 0;
    var inc_count_2: i32 = 0;

    var x: [3]i32 = undefined;

    for ([_]usize{ 0, 1, 2 }) |i| {
        var line = try in_stream.readUntilDelimiterOrEof(&buf, '\n');
        x[i] = try std.fmt.parseInt(i32, line.?, 10);
    }

    if (x[0] < x[1])
        inc_count_1 += 1;
    if (x[1] < x[2])
        inc_count_1 += 1;

    var idx: usize = 0;
    var prev_idx: usize = 2;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var sum_1 = x[0] + x[1] + x[2];

        x[idx] = try std.fmt.parseInt(i32, line, 10);

        if (x[prev_idx] < x[idx])
            inc_count_1 += 1;

        prev_idx = idx;
        idx += 1;
        idx %= 3;

        var sum_2 = x[0] + x[1] + x[2];

        if (sum_1 < sum_2)
            inc_count_2 += 1;
    }

    print("{}\n{}\n", .{ inc_count_1, inc_count_2 });
}
