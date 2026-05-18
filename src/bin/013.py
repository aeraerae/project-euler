import sys
#from math import gcd, isqrt, floor, sin, pi
#from collections import deque
#import random
 
def input(): return sys.stdin.readline().rstrip()
def ints(): return map(int, input().split())
def iint(): return int(input())
 
# 새 계명을 너희에게 주노니 서로 사랑하라 내가 너희를 사랑한것 같이 너희도 서로 사랑하라 (요한복음 13:34)
# https://youtu.be/r89YusWbFZE
def solve():
    ans = 0
    for _ in range(100): ans += iint()
    print(str(ans)[:10])
 
#for _ in range(iint()): solve()
solve()