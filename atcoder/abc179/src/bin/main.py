MOD = 998244353

dp_acc = [0] * (2 * N + 1)


def dp_range(l, r):
    left = 0 if l-1 < 0 else dp_acc[l-1]
    right = 0 if r < 0 else dp_acc[r]
    return right - left


N, K = map(int, input().split())
segments = [tuple(map(int, input().split())) for _ in range(K)]
dp = [0] * (2 * N + 1)
dp[0], dp_acc[0] = 1, 1


for i in range(n):
    for j in range(k):
        dp[i] += dp_range(i - segment[j][1], i - segment[j][0])
        dp[i] %= MOD
        dp_acc[i] = (dp_acc[i - 1] + dp[i]) % MOD
print(dp[N-1])
