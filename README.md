# Montgomery-Multiplication
This is a method for performing fast modular multiplication by transforming numbers into a special Montgomery form, avoiding costly division operations.


// This is Montgomery Multiplication: The aim is to calculate the product a.b%p for two integers a and b modulo p, where p is a large prime. 
// Why: It is costly to apply %p in the code
// finding the Montogomery representations of a and b with respect to an integer r>p is usually helpful in calculating a.b%p. 
// The following link provide a good explanation to the algorithm:   https://www.nayuki.io/page/montgomery-reduction-algorithm

// For a single multiplication, Montgomery is inferior to modular multiplication. 
// But for a chain of multiplications, such as in modular exponentiation, we transform the input numbers into Montgomery form, 
// perform numerous multiplications, and transform back to standard numbers at the end.

//**Detailed algorithm**
// **Initialization:**
// Problem: We want to compute many instances of c≡a×b mod n. We will compute with many values of a and b but keep n fixed.
// Choose an r that is greater than n and coprime with n. Typically we let r be a power of 2, which means n needs to be odd and greater than 3.
// Later in the algorithm, we will perform modulo by r and division by r. With r being a power of 2, these operations respectively become inexpensive bit masking and right shifting.
// If the base is 10 due to performing decimal arithmetic by hand or using BCD, then r should accordingly be a power of 10. The reasoning behind the algorithm is still all valid.
// Let k=(r(r^{−1} mod n)−1)/(n) . (This division is exact.)
// The reciprocal r^{−1} mod n exists because r is coprime with n. We know that r*(r^{−1} mod n)≡1 mod n. Thus r*(r^{−1} mod n)=1+kn for some non-negative integer k.
// r^{−1} mod n is computed by the extended Euclidean algorithm.

// **Outer algorithm:**
// Since we are doing arithmetic modulo n, we assume that all input and output numbers are in the range [0,n). Intermediate results in the algorithm may be larger but not negative.
// In this discussion, we carefully distinguish between equations on integers (x=y) versus congruences of integers modulo a number (x≡y mod n). We also distinguish the use of mod as an arithmetic operator versus its use in congruence relations.

// First convert the input numbers to Montgomery form: \bar{a}=(a*r mod n), \bar{b}=(b*r mod n).
// Assume that our desired output in Montgomery form is \bar{c}=(c*r mod n)=(a*b*r mod n).
// Compute the product directly, so we have: \bar{a}*\bar{b}≡(a*r)(b*r)≡a*b*r^2≡\bar{c}*r mod n.
// Note that 0≤\bar{a}*\bar{b}<n^2.
// Now compute the reduction: \bar{c}= R(\bar{a}*\bar{b})=(a*b*r mod n), where R() is the reduction function described below
// Finally convert the output back to standard form: c=(\bar{c}*r^{−1} mod n).

// **Reduction function**
// The reduction function R:[0,n^2)→[0,n) effectively computes R(x)=(x*r^{−1} mod n) in an efficient way.
// Let s=(x*k mod r). (k is defined in the initialization.)
// We know that 0≤s<r. By the properties of modular arithmetic, we can replace x in this expression with (x mod r) to get the same result more efficiently.
// Let t=x+s*n.
// We know that 0≤x<n^2<r*n and 0≤s*n<r*n, thus 0≤t<2r*n. (Recall that we chose r>n.)

// We claim that t is a multiple of r, with proof: x+s*n≡ x+x*k*n≡x(1+kn)≡x(r(r^{−1} mod n))≡0 mod r.
// Therefore we can write t=u*r for some non-negative integer u.
// We know that t≡x mod n because adding s*n preserves the congruence.

// Let u=t/r. (This division is exact.)
// We can see that u≡u(r(r^{−1} mod n))≡(u*r)(r^{−1} mod n)≡t(r^{−1} mod n) ≡ x(r^{−1} mod n) mod n.

// If 0≤u<n then return u. Otherwise return u−n.
// Because 0≤t<2rn, we have 0≤u<2n. Hence this simple if-else performs a modulo by n correctly. Furthermore, the result is still equal (and congruent) to x(r^{−1} mod n) mod n.

// This completes the explanation and proof of the Montgomery reduction algorithm.
