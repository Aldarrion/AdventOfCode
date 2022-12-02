const print = @import("std").debug.print;
const std = @import("std");

fn compute(memory: []u32) u32 {
    var ip: u32 = 0;
    while (memory[ip] != 99) {
        var op = memory[ip];
        if (op == 1) {
            memory[memory[ip + 3]] = memory[memory[ip + 1]] + memory[memory[ip + 2]];
        } else if (op == 2) {
            memory[memory[ip + 3]] = memory[memory[ip + 1]] * memory[ memory[ip + 2]];
        }
        ip += 4;
    }

    return memory[0];
}

pub fn main() !void {
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    // Result of `try ...` is an optional. |variable_name| syntax unwraps the optional to a variable when valid
    // otherwise it ends the while loop
    var data = (try in_stream.readUntilDelimiterOrEof(&buf, '\n')).?;
    var memory_init = [_]u32{ 0 } ** (4 * 1024);
    var iter = std.mem.split(u8, std.mem.trim(u8, data, &std.ascii.spaces), &[_]u8{ ',' });
    var num = iter.next();
    var i: u32 = 0;
    while (num) |n| {
        memory_init[i] = (try std.fmt.parseUnsigned(u32, n, 10));
        i += 1;
        num = iter.next();
    }

    var memory = [_]u32{ 0 } ** (4 * 1024);
    std.mem.copy(u32, &memory, &memory_init);
    memory[1] = 12;
    memory[2] = 2;
    var res1 = compute(&memory);

    var n: u32 = 0;
    var v: u32 = 0;
    outer: while (n < 100) : (n += 1) {
        while (v < 100) : (v += 1) {
            std.mem.copy(u32, &memory, &memory_init);
            memory[1] = n;
            memory[2] = v;
            var res = compute(&memory);
            if (res == 19690720) {
                break: outer;
            }
        }
        v = 0;
    }

    print("{}\n(100 * {} + {}) = {}\n", .{ res1, n, v, (100 * n + v) });
}
