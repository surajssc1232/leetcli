// Maximum Profit with K Transactions
// You are given an array of stock prices `prices` where `prices[i]` is the price of a given stock on the i-th day.
// You are also given an integer `k` which represents the maximum number of transactions you are allowed to make.
// A transaction consists of buying and selling one share of the stock. You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
// Your goal is to find the maximum profit you can achieve.

// Example 1:
// Input: k = 2, prices = [2,4,1]
// Output: 2
// Explanation: Buy on day 1 (price = 2) and sell on day 2 (price = 4), profit = 4-2 = 2.
// Then buy on day 3 (price = 1) and sell on day 3 (price = 1), profit = 1-1 = 0. Total profit = 2.

// Example 2:
// Input: k = 2, prices = [3,2,6,5,0,3]
// Output: 7
// Explanation: Buy on day 2 (price = 2) and sell on day 3 (price = 6), profit = 6-2 = 4.
// Then buy on day 5 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3. Total profit = 4+3 = 7.

function maxProfit(k, prices) {
    // TODO: implement your solution here
    // TODO: implement
}

function runTests() {
    const testCases = [
        { k: 2, prices: [2, 4, 1], expected: 2 },
        { k: 2, prices: [3, 2, 6, 5, 0, 3], expected: 7 },
        { k: 1, prices: [7, 1, 5, 3, 6, 4], expected: 5 }, // Buy at 1, sell at 6
        { k: 3, prices: [1, 2, 3, 4, 5], expected: 4 }, // Buy 1 sell 5
        { k: 0, prices: [1, 2, 3, 4, 5], expected: 0 }, // No transactions allowed
        { k: 5, prices: [1, 2, 3, 4, 5], expected: 4 }, // k > number of potential profitable transactions
        { k: 1, prices: [5, 4, 3, 2, 1], expected: 0 }, // Decreasing prices
        { k: 2, prices: [], expected: 0 }, // Empty prices array
        { k: 1, prices: [10], expected: 0 }, // Single price
        { k: 3, prices: [1, 2, 1, 2, 1, 2, 1, 2, 1, 2], expected: 5 }, // Alternating up/down
        { k: 1000, prices: [8,6,4,3,3,2,3,5,8,3,8,2,3,4,6,8], expected: 13 }, // Large k, complex price movements
    ];
    
    let passed = 0;
    testCases.forEach((testCase, i) => {
        const { k, prices, expected } = testCase;
        try {
            const result = maxProfit(k, [...prices]); // Use a copy to prevent modification
            if (result === expected) {
                console.log(`✓ Test ${i+1} PASSED`);
                passed++;
            } else {
                console.log(`✗ Test ${i+1} FAILED: k=${k}, prices=[${prices.join(',')}] | expected ${expected}, got ${result}`);
            }
        } catch (e) {
            console.log(`✗ Test ${i+1} ERROR: k=${k}, prices=[${prices.join(',')}] | ${e.message}`);
        }
    });
    
    console.log(`\nResult: ${passed}/${testCases.length} tests passed`);
    // TODO: implement
}

runTests();