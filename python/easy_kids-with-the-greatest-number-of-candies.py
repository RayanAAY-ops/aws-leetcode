class Solution:
    def kidsWithCandies(self, candies: List[int], extraCandies: int) -> List[bool]:
        res = []
        for x in candies:
            res.append(x + extraCandies >= max(candies))
        return res

