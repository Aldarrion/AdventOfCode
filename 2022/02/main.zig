const print = @import("std").debug.print;
const std = @import("std");

fn calculateScore1(me: u8, opponent: u8) i32 {
    var score: i32 = 0;
    switch (me) {
        'X' => {
            score = 1;
            score += switch (opponent) {
                'A' => 3,
                'B' => 0,
                'C' => 6,
                else => 0,
            };
        },
        'Y' => {
            score = 2;
            score += switch (opponent) {
                'A' => 6,
                'B' => 3,
                'C' => 0,
                else => 0,
            };
        },
        'Z' => {
            score = 3;
            score += switch (opponent) {
                'A' => 0,
                'B' => 6,
                'C' => 3,
                else => 0,
            };
        },
        else => {},
    }
    return score;
}

fn symbolToNumber(symbol: u8) i32 {
    return switch (symbol) {
        'A', 'X' => 1,
        'B', 'Y' => 2,
        'C', 'Z' => 3,
        else => 0
    };
}

fn calculateScore1Short(me: i32, opponent: i32) i32 {
    var res = [_]i32{ 3, 0, 6 };
    var score = me;
    score += res[@intCast(usize, @mod((opponent - 1 - (me - 1)), 3))];
    return score;
}

fn calculateScore2(me: u8, opponent: u8) i32 {
    var score: i32 = 0;
    switch (me) {
        'X' => { // Lose
            score = 0;
            score += switch (opponent) {
                'A' => 3, // Scissors
                'B' => 1, // Rock
                'C' => 2, // Paper
                else => 0,
            };
        },
        'Y' => { // Draw
            score = 3;
            score += switch (opponent) {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                else => 0,
            };
        },
        'Z' => { // Win
            score = 6;
            score += switch (opponent) {
                'A' => 2, // Paper
                'B' => 3, // Scissors
                'C' => 1, // Rock
                else => 0,
            };
        },
        else => {},
    }
    return score;
}

fn calculateScore2Short(me: i32, opponent: i32) i32 {
    var res = [_]i32{ 3, 1, 2 };
    var scores = [_]i32{ 0, 3, 6 };
    var score = scores[@intCast(usize, me - 1)];
    score += res[@intCast(usize, @mod((opponent - 1 + (me - 1)), 3))];
    return score;
}

pub fn main() !void {
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    var total_score_1: i32 = 0;
    var total_score_2: i32 = 0;
    var total_score_1_short: i32 = 0;
    var total_score_2_short: i32 = 0;

    // Result of `try ...` is an optional. |variable_name| syntax unwraps the optional to a variable when valid
    // otherwise it ends the while loop
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line_raw| {
        var line = line_raw;
        total_score_1 += calculateScore1(line[2], line[0]);
        total_score_1_short += calculateScore1Short(symbolToNumber(line[2]), symbolToNumber(line[0]));
        total_score_2 += calculateScore2(line[2], line[0]);
        total_score_2_short += calculateScore2Short(symbolToNumber(line[2]), symbolToNumber(line[0]));
    }

    print("{}\n{}\n{}\n{}\n", .{ total_score_1, total_score_1_short, total_score_2, total_score_2_short });
}
