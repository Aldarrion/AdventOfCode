const print = @import("std").debug.print;
const std = @import("std");

fn processLine(line: []u8, ones: [*]i32) void {
    for (line) |c, i| {
        if (c == '1') {
            ones[i] += 1;
        }
    }
}

pub fn main() !void {
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    var count: i32 = 0;
    var ones = [_]i32{0} ** 32;

    var bit_count: u5 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        bit_count = @intCast(u5, line.len);
        processLine(line, &ones);
        count += 1;
    }

    var gamma: u32 = 0;
    for (ones) |ones_count, i| {
        if (ones_count > @divTrunc(count,  2)) {
            gamma |= (@intCast(u32, 1) << @intCast(u5, bit_count - i - 1));
        }
    }

    var bit_complement = @intCast(u5, 32 - @intCast(u32, bit_count));
    var mask = (((~@intCast(u32, 0)) << bit_complement) >> bit_complement);
    var epsilon = (~gamma) & mask;

    in_stream = buf_reader.reader();
    var data: [32*2000]u8 = undefined;

    var read = try in_stream.readAll(&data);
    std.debug.assert(read < data.len);

    var replace_size = std.mem.replace(u8, &data, "\n", "", &data);
    std.debug.assert(replace_size < data.len);

    var indices = std.ArrayList(i32).init(std.heap.c_allocator);
    {
        var i: i32 = 1;
        while (i < count) : (i += 1) {
            try indices.append(i);
        }
    }

    {
        var bit_idx: i32 = 0;
        while (indices.items.len > 1) {
            var one_count: i32 = 0;
            for (indices.items) |idx| {
                var data_idx = @intCast(usize, idx * bit_count + bit_idx);
                if (data[data_idx] == '1') {
                    one_count += 1;
                }
            }

            var char: u8 = '0';
            if (one_count >= indices.items.len) {
                char = '1';
            }

            var i: usize = 0;
            while (i < indices.items.len) {
                var data_idx = @intCast(usize, indices.items[i] * bit_count + bit_idx);
                if (data[data_idx] != char) {
                    _ = indices.swapRemove(i);
                } else {
                    i += 1;
                }
            }

            bit_idx += 1;
        }
    }

    {
        var idx = indices.items[0];
        var data_idx = @intCast(usize, idx * bit_count);
        var bit_idx: i32 = 0;
        var num: u32 = 0;
        while (bit_idx < bit_count) {
            if (data[data_idx] == '1') {
                num |= (@intCast(u32, 1) << @intCast(u5, bit_count - bit_idx - 1));
            }
            bit_idx += 1;
        }
        print("{}\n", .{ num });
    }


    //print("{b}\n{b} {}\n{b} {}\n{}", .{mask, gamma, gamma, epsilon, epsilon, count});

    print("{}\n", .{@intCast(u64, gamma) * @intCast(u64, epsilon)});
}
