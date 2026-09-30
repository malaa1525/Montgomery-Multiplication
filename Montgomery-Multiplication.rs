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






fn main(){
    let n: u64 = 4294967291; // largest prime less than 2^{32}-1
    let i: u8 = 32; // i is the power of 2 for which we take r=2^i. // i should be chosen so that we it is less than 128, since u128 is the largest usgined type in Rust
    let r = 1u64 << i; // the operator << is left shift, which multiplies by powers of 2. That's why r=1u64 << 32 is 2³² // r could be felixable, ranging from 2 to 2^127, but r>n by assumption
    
    
    //calculating r_inverse mod n
    let r_inverse= modular_inverse(r,n);
    let k=(r*r_inverse-1)/n;
    //println!("r^-1={r_inverse}, k={k}"); // r^-1=3435973833,    k=3435973837
    
    
    let mut a: u64 = 123698745; //choose any integer in range 0 -- r-1 for both a and b
    let mut b: u64 = 4294967295 - 3658921736;
    // Calculating the montgomery_form for a and b
    a=a*r%n;
    b=b*r%n; // we get a=618493725, b=3180227795
    //println!("a_bar * b_bar={}",a*b);
    let c=reduction(reduction(a*b, k, n), k, n); // the two sequential reductions mentioned above
    println!("a*b%n={c}");
    
}


fn modular_inverse(r: u64, n: u64) -> u64{
    let a: i128 =r as i128;
    let b: i128=n as i128;
    let u: i128 =1;
    let g: i128 =a;
    let x: i128=0; 
    let y: i128 =b;
    let mut r_inverse= gcd_extended(a,b,g,u,x,y).1;
    if r_inverse < 0 {
        r_inverse+=n as i128;
    }
    let r_inverse=r_inverse as u64;
    return r_inverse
}

fn gcd_extended(a: i128, b: i128, g: i128, u: i128, x: i128, y:i128) -> (i128, i128, i128){
    if y ==0 {return (g,u,(g-a*u)/b)}
    let q= g/y as i128;
    let t= g-q*y;
    let s =u-q*x;
    let u =x;
    let g =y; 
    let x =s;
    let y =t;
    return gcd_extended(a,b,g,u,x,y)
}


fn reduction(x: u64, k: u64, n: u64) -> u64 {
    let s = (x as u32).wrapping_mul(k as u32) as u64; // (x mod 2^32)*(k mod 2^32) mod 2^32 // wrapping_mul() is multiplication with wrapping semantics — meaning that if the result overflows the integer type’s range, it wraps around using two’s complement rules instead of panicking or saturating. 
    let t = x as u128 + (s as u128) * (n as u128);           // t < 2rn < 2^65
    let u = (t >> 32) as u64;                                // exact division by r // >> is the right-shift operator. t >> 32 moves all the bits of t 32 places to the right and discards the 32 bits that fall off the end. In numbers, that's floor(t / 2³²). It's the binary version of dropping the last three digits of a decimal number to divide by 1000.
                                                            // In the algorithm, we showed that t is a multiple of r, so its low 32 bits are all zero. That means t >> 32 is exactly t/r, with nothing lost to the floor. This is the "division by r is just a right shift" step, which is why r is chosen to be a power of 2.
    if u < n { u } else { u - n }
}
