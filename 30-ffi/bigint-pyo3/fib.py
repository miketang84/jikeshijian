def fibonacci(n):
    if n <= 0:
        return "输入的数字必须大于0"
    elif n == 1:
        return 0
    elif n == 2:
        return 1
    else:
        a, b = 0, 1
        for _ in range(n - 2):
            a, b = b, a + b
        return b

# 测试函数
# print(fibonacci(100000))
fibonacci(2000000)

