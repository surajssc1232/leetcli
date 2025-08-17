# Trapping Rain Water II
# You are given an m x n integer matrix heightMap represents the height of each unit cell in a 2D grid.
# Return the volume of water it can trap after raining.
#
# Example 1:
# Input: heightMap = [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]
# Output: 4
#
# Example 2:
# Input: heightMap = [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]
# Output: 10

import heapq

def trapRainWater(heightMap):
    # TODO: implement
    pass

if __name__ == "__main__":
    # Test cases
    assert trapRainWater([[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]) == 4
    assert trapRainWater([[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]) == 10
    assert trapRainWater([[12,13,1,12],[13,4,13,12],[13,8,10,12],[12,13,12,12],[13,13,13,13]]) == 14
    assert trapRainWater([[1]]) == 0
    assert trapRainWater([[1,2],[3,4]]) == 0
    print("All tests passed!")
