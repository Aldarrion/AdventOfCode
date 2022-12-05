const print = @import("std").debug.print;
const std = @import("std");

const Range = struct {
    min: i32,
    max: i32,
};

pub fn main() !void {
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    var num_contained: i32 = 0;
    var num_overlapping: i32 = 0;

    var line_number: i32 = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line_raw| {
        line_number += 1;
        var line = std.mem.trim(u8, line_raw, &std.ascii.spaces);
        var ranges_str = std.mem.split(u8, line, &[_]u8{ ',' });
        var ranges: [2]Range = undefined;
        var range_iter = ranges_str.next();
        var i: usize = 0;
        while (range_iter) |r| {
            var num_iter = std.mem.split(u8, r, &[_]u8{ '-' });
            ranges[i].min = try std.fmt.parseInt(i32, num_iter.next().?, 10);
            ranges[i].max = try std.fmt.parseInt(i32, num_iter.next().?, 10);
            range_iter = ranges_str.next();
            i += 1;
        }

        var x = ranges[0];
        var y = ranges[1];
        if ((x.min >= y.min and x.max <= y.max)
            or (y.min >= x.min and y.max <= x.max))
        {
            num_contained += 1;
        }

        if (x.max >= y.min and x.min <= y.max) {
            num_overlapping += 1;
        }
    }

    print("{}\n{}\n", .{ num_contained, num_overlapping });
}
