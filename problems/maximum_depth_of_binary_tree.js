// Maximum Depth of Binary Tree
// Given the root of a binary tree, return its maximum depth.
// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
//
// Example 1:
// Input: root = [3,9,20,null,null,15,7]
// Output: 3
//
// Example 2:
// Input: root = [1,null,2]
// Output: 2

/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
function maxDepth(root) {
    // TODO: implement
}

// Helper function to create a binary tree from an array representation
function arrayToTree(arr, i = 0) {
    if (i >= arr.length || arr[i] === null) {
        // TODO: implement
    }
    let node = { val: arr[i], left: null, right: null };
    node.left = arrayToTree(arr, 2 * i + 1);
    node.right = arrayToTree(arr, 2 * i + 2);
    // TODO: implement
}

// Test cases
function runTests() {
    let root1 = arrayToTree([3, 9, 20, null, null, 15, 7]);
    console.log(`Input: [3,9,20,null,null,15,7], Output: ${maxDepth(root1)}`); // Expected: 3

    let root2 = arrayToTree([1, null, 2]);
    console.log(`Input: [1,null,2], Output: ${maxDepth(root2)}`); // Expected: 2

    let root3 = arrayToTree([]);
    console.log(`Input: [], Output: ${maxDepth(root3)}`); // Expected: 0

    let root4 = arrayToTree([0]);
    console.log(`Input: [0], Output: ${maxDepth(root4)}`); // Expected: 1

    let root5 = arrayToTree([1, 2, 3, 4, 5]);
    console.log(`Input: [1,2,3,4,5], Output: ${maxDepth(root5)}`); // Expected: 3
}

runTests();