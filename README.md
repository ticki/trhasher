# Trhasher

Trhasher is an extensive hash function quality-and-performance-test. It
analysis the function with multiple method, eventually trashing the hash
function.

## The tests

Trhasher doesn't have particularly many tests. What makes it powerful is how it
combines them.

It uses the following data sets:

1. The English dictionary.
2. A list of primes.
3. Random numbers.
4. Random ASCII text.
5. Low quality randomness stream.
6. Counting numbers.

Each of these are tested with the hashing function, combined with a,
potentially, entropy-reducing bijective function. This way we can notice
patterns that wasn't obvious if we simply analyzed it directly. The following
transforming functions are used:

1. Identity (i.e., no function applied).
2. XOR fold (i.e., XOR adjacent hashes in the stream), this is good at
   detecting consecutive duplicates in the stream.
3. Addition fold (i.e., add adjacent hashes in the stream), this makes additive
   patterns more obvious.
4. Prime multiplication (i.e., multiply the hashes by some prime), this
   exploits a commonly used technique in hash functions, to find patterns.
5. Double hashing (i.e., perform hashing twice), bad hashing functions tends to
   let this make the quality _worse_ than hashing it once. This transform makes
   sure that's not the case.
6. Hadamard transform, in repetitive or low-entropy sequences, Hadamard
   transforms often makes them very biased, making it easier to detect.
7. Jump over (i.e., skip every two numbers in the stream), a common newbie
   mistake is to zip hashing functions, under the wrong assumption that it is
   better that way. This transform makes those cases more obvious, by
   unzipping, fully or partially, the hasher.

Each transformed stream is then tested through multiple parameters:

1. The chi-squared distribution of bytes. This rules out the most obvious biased.
2. The coverage of bytes. This checks if certain bytes are impossible or very
   unlikely to get.
3. Bit fairness. This makes sure that the bits are fairly chosen.
4. Maximal buckets collisions. This keeps an array of 4096 elements, and
   increments each based on the hash modulo 4096. The maximal value should be
   kept as low as possible to avoid collisions and bucket overflow.
5. Buckets filled. A test similar to the one described above, is done. The
   number of filled buckets are outputted.
6. The chi-squared distribution of the bucket counts.
7. The average value in the hash stream.
6. The AND zero test. This tests how many hash values, you need to AND before
   you reach zero.

Lastly, we have a bunch of generic tests, which doesn't need a particular data
set:

1. Rehashing test. This produces a random-number generator based on hashing the
   RNG state, and then test it via the methods described above.
2. Zero sensitivity test. This makes sure that the hash function is zero
   sensitive. A poor hash function doesn't distinguish between e.g. H(10010100)
   and H(1001010).
3. Determinism test. This tests makes sure that the hash function is pure and
   deterministic.

There are a few profiling parameters as well:

1. GB/s. Tests how many gigabytes that can be hase d each second.
2. Total time spend.
3. Time spend on each test.
