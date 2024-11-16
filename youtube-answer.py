import unittest

class Solution:
    def minPathSum(self, grid: list[list[int]]) -> int:
        m,n = len(grid), len(grid[0])

        for i in range(m):
            for j in range(n):
                if i == j == 0:
                    continue
                
                left_path = up_path = float('inf')
                if i != 0:
                    up_path = grid[i][j] + grid[i-1][j]
                if j != 0:
                    left_path = grid[i][j] + grid[i][j-1]
                
                grid[i][j] = min(up_path, left_path)
        
        return grid[m-1][n-1]

class TestSolution(unittest.TestCase):
    def test_minPathSum(self):
        solution = Solution()
        grid = [[1,3,1],[1,5,1],[4,2,1]]
        self.assertEqual(solution.minPathSum(grid), 7)
        grid = [[1,2,3],[4,5,6]]
        self.assertEqual(solution.minPathSum(grid), 12)

if __name__ == '__main__':
    unittest.main()
