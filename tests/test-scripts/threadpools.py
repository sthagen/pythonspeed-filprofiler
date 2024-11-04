"""Validate that number of threads in thread pools is set to 1."""

import numexpr

try:
    import blosc
except ImportError:
    blosc = None
import threadpoolctl

# APIs that return previous number of threads:
assert numexpr.set_num_threads(2) == 1
if blosc is not None:
    assert blosc.set_nthreads(2) == 1

for d in threadpoolctl.threadpool_info():
    assert d["num_threads"] == 1, d
