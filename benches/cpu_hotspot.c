/* Standalone synthetic workload, not production profiler code. */
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

#if defined(__GNUC__) || defined(__clang__)
#define NOINLINE __attribute__((noinline))
#else
#error "This benchmark requires a compiler with a noinline attribute"
#endif

/* Identical dependent operations and a 7:2:1 iteration ratio approximate
 * 70%/20%/10% CPU work. Actual sampled shares depend on compiler and hardware.
 * Unsigned arithmetic wraps intentionally; printing the result keeps it live.
 */
NOINLINE uint64_t hotspot_70(uint64_t state) {
    for (uint64_t i = 0; i < UINT64_C(7000000); ++i) {
        state = (state ^ (state >> 13)) * UINT64_C(6364136223846793005) + i;
    }
    return state;
}

NOINLINE uint64_t hotspot_20(uint64_t state) {
    for (uint64_t i = 0; i < UINT64_C(2000000); ++i) {
        state = (state ^ (state >> 13)) * UINT64_C(6364136223846793005) + i;
    }
    return state;
}

NOINLINE uint64_t hotspot_10(uint64_t state) {
    for (uint64_t i = 0; i < UINT64_C(1000000); ++i) {
        state = (state ^ (state >> 13)) * UINT64_C(6364136223846793005) + i;
    }
    return state;
}

int main(void) {
    uint64_t state = 1;
    for (unsigned int round = 0; round < 100; ++round) {
        state = hotspot_70(state);
        state = hotspot_20(state);
        state = hotspot_10(state);
    }
    printf("checksum: %" PRIu64 "\n", state);
    return 0;
}
