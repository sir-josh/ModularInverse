# Modular Multiplicative Inverse using Euclidean Algorithm.
###### [Link to code solution] (/src/main.rs)

## What is multiplicative inverse? 
A multiplicative inverse of a number **x** is a another number that will multiply **x** to get 1.  i.e **1/x**

## What is modular multiplicative inverse ?
A modular multiplicative inverse of an integer A is an integer X such that the product AX is congruent to 1 with respect to the modulus M. 

##           AX ≅ 1 mod M

> Note: The value of X should be in the range {1, 2, … M-1}, i.e., in the range of integer modulo M. ( Note that X cannot be 0 as A*0 mod M will never be 1). The  multiplicative inverse of “A modulo M” exists if and only if A and M are relatively prime (i.e. if gcd(A, M) = 1)


## **For Example**
3X ≅ 1 mod 17. What is the value of X?
```
Input:  A = 3, M = 17
Output: 6
Explamation: Since (3*6) mod 17 = 1, 6 is modulo inverse of 10(under 17).
```
## **Another Example**
4X ≅ 1 mod 11. What is the value of X?

```
Input: A = 4, M = 11
Output: 3
Explanation: Since (4*3) mod 11 = 1, 4 is modulo inverse of 3(under 11).
One might think, 15 also as a valid output as “(15*3) mod 11” 
is also 1, but 15 is not in range {1, 2, … 10}, so not valid.
```


## Finding Modular Multiplicative Inverse using Extended Euclidean Algorithm
The idea is to use **Extended Euclidean algorithms** that take two integers ‘a’ and ‘b’, then find their gcd, and also find **‘x’** and **‘y’** such that 

**ax + by = gcd(a, b)**

To find the multiplicative inverse of ‘A’ under ‘M’, we put b = M in the above formula. Since we know that A and M are relatively prime, we can put the value of gcd as 1.

**Ax + My = 1**

If we take modulo M on both sides, we get

**Ax + My ≅ 1 (mod M)**

We can remove the second term on left side as **‘My (mod M)’** would always be 0 for an integer y. 

**Ax  ≅ 1 (mod M)**

So the ‘x’ that we can find using Extended Euclid Algorithm is the multiplicative inverse of ‘A’

> Note: By reversing the steps in the Euclidean algorithm, it is possible to find these integers X and Y. ( The whole idea is to start with the GCD and recursively work our way backwards)
