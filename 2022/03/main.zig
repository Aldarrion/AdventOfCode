const print = @import("std").debug.print;
const std = @import("std");

pub fn main() !void {
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    const type_count: u8 = 'z' - 'A' + 1;

    var sum_duplicates: usize = 0;
    var sum_badges: usize = 0;

    var line_number: i32 = 0;
    var group_types = std.mem.zeroes([type_count]u8);
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line_raw| {
        line_number += 1;
        var line = std.mem.trim(u8, line_raw, &std.ascii.spaces);

        {
            var types = std.mem.zeroes([type_count]u8);
            var i: usize = 0;
            while (i < line.len / 2) {
                var idx = line[i] - 'A';
                types[idx] += 1;
                i += 1;
            }

            while (i < line.len) {
                var idx = line[i] - 'A';
                var c = line[i];
                if (types[idx] > 0) {
                    var prio = if (c >= 'a' and c <= 'z')
                        (c - 'a' + 1)
                    else
                        (26 + c - 'A' + 1);
                    sum_duplicates += prio;
                    types[idx] = 0;
                }
                i += 1;
            }
        }

        var full_types = std.mem.zeroes([type_count]u8);
        for (line) |c| {
            var idx = c - 'A';
            full_types[idx] += 1;
            if (full_types[idx] == 1) {
                group_types[idx] += 1;
            }
        }

        // Group
        if (@mod(line_number, 3) == 0) {
            for (group_types) |t, i| {
                if (t == 3) {
                    var c = @intCast(u8, i) + 'A';
                    var prio = if (c >= 'a' and c <= 'z')
                        (c - 'a' + 1)
                    else
                        (26 + c - 'A' + 1);
                    sum_badges += prio;
                }
            }
            group_types = std.mem.zeroes([type_count]u8);
        }
    }

    print("{}\n{}\n", .{ sum_duplicates, sum_badges });
}
