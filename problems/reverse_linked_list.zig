// Reverse Linked List

// TODO: implement

// Definition for singly-linked list.
const ListNode = struct {
    val: i32,
    next: ?*ListNode,
};

pub fn reverseList(head: ?*ListNode) ?*ListNode {
    // TODO: implement your solution here
    

}

const std = @import("std");

fn buildList(allocator: std.mem.Allocator, values: []const i32) !?*ListNode {
    // TODO: implement
    var head = try allocator.create(ListNode);
    head.val = values[0];
    head.next = null;
    var current = head;
    for (values[1..]) |val| {
        const new_node = try allocator.create(ListNode);
        new_node.val = val;
        new_node.next = null;
        current.next = new_node;
        current = new_node;
    }
    // TODO: implement
}

fn listToArray(allocator: std.mem.Allocator, head: ?*ListNode) ![]i32 {
    var values = std.ArrayList(i32).init(allocator);
    var current = head;
    while (current) |node| {
        try values.append(node.val);
        current = node.next;
    }
    // TODO: implement
}

fn freeList(allocator: std.mem.Allocator, head: ?*ListNode) void {
    var current = head;
    while (current) |node| {
        const next = node.next;
        allocator.destroy(node);
        current = next;
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const final_leak_count = gpa.deinit();
        if (final_leak_count > 0) {
            std.debug.print("LEAK DETECTED: {d} allocations\n", .{final_leak_count});
        }
    }
    const allocator = gpa.allocator();

    var tests = std.ArrayList(struct {
        input_values: []const i32,
        expected_values: []const i32,
        name: []const u8,
    }).init(allocator);
    defer {
        // Freeing the input_values and expected_values slices.
        // Note: These slices are copies from string literals, so they don't need explicit freeing.
        // If they were dynamically allocated, we would free them here.
    }

    try tests.append(.{ .input_values = &[_]i32{1, 2, 3, 4, 5}, .expected_values = &[_]i32{5, 4, 3, 2, 1}, .name = "Example 1" });
    try tests.append(.{ .input_values = &[_]i32{1, 2}, .expected_values = &[_]i32{2, 1}, .name = "Example 2" });
    try tests.append(.{ .input_values = &[_]i32{}, .expected_values = &[_]i32{}, .name = "Example 3 Empty List" });
    try tests.append(.{ .input_values = &[_]i32{1}, .expected_values = &[_]i32{1}, .name = "Single Node" });
    try tests.append(.{ .input_values = &[_]i32{-1, -2, -3}, .expected_values = &[_]i32{-3, -2, -1}, .name = "Negative Numbers" });
    try tests.append(.{ .input_values = &[_]i32{0, 0, 0}, .expected_values = &[_]i32{0, 0, 0}, .name = "All Zeros" });

    var passed_count = 0;
    var failed_count = 0;

    for (tests.items) |test| {
        const input_head = try buildList(allocator, test.input_values);
        const reversed_head = reverseList(input_head);
        const actual_values = try listToArray(allocator, reversed_head);

        if (std.mem.eql(i32, actual_values, test.expected_values)) {
            std.debug.print("{s}: PASS\n", .{test.name});
            passed_count += 1;
        } else {
            std.debug.print("{s}: FAIL - Expected: {s}, Got: {s}\n", .{
                test.name,
                std.fmt.allocPrint(allocator, "{s}", .{test.expected_values}).?,
                std.fmt.allocPrint(allocator, "{s}", .{actual_values}).?,
            });
            failed_count += 1;
        }

        freeList(allocator, input_head);
        freeList(allocator, reversed_head); // Free the reversed list as well
        allocator.free(actual_values);
    }

    std.debug.print("\n--- Test Summary ---\n", .{});
    std.debug.print("Total Tests: {d}\n", .{tests.items.len});
    std.debug.print("Passed: {d}\n", .{passed_count});
    std.debug.print("Failed: {d}\n", .{failed_count});
}
