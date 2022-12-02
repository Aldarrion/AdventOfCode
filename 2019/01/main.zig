const print = @import("std").debug.print;
const std = @import("std");

fn calcFuel(mass: i32) i32 {
    return @divFloor(mass, 3) - 2;
}

pub fn main() !void {
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    var module_fuel_sum: i64 = 0;
    var additional_fuel_sum: i64 = 0;

    // Result of `try ...` is an optional. |variable_name| syntax unwraps the optional to a variable when valid
    // otherwise it ends the while loop
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var line_trimmed = std.mem.trim(u8, line, &std.ascii.spaces);
        var mass = try std.fmt.parseInt(i32, line_trimmed, 10);
        var fuel_requirement = calcFuel(mass);
        module_fuel_sum += fuel_requirement;

        var add_fuel = calcFuel(fuel_requirement);
        while (add_fuel > 0) {
            additional_fuel_sum += add_fuel;
            fuel_requirement = add_fuel;
            add_fuel = calcFuel(fuel_requirement);
        }
    }

    print("{}\n{}\n", .{ module_fuel_sum, module_fuel_sum + additional_fuel_sum });
}
