import os
import pprint
import sys
import time

import md_ulb_pwrap as rs_md_ulb_pwrap

sys.path.insert(0, os.path.abspath("src"))
import py_md_ulb_pwrap

TIMEOUT = 500  # ms
WARN_TIMEOUT = 300

IMPLEMENTATIONS = [
    ("Python", py_md_ulb_pwrap),
    ("Rust", rs_md_ulb_pwrap),
]

FUNCTIONS = [
    (
        "ulb_wrap_paragraph",
        2000,
        (
            (
                ("aaa ``` ``  ` a b c ``` ccc", 3, 3),
                "aaa\n``` ``  ` a b c ```\nccc",
            ),
            (
                ("\n\n\naa bb cc\n\n\n", 2, 2),
                "\n\n\naa\nbb\ncc\n\n\n",
            ),
        ),
    ),
]

EXITCODE = 0

for func_name, iterations, args_expected_results in FUNCTIONS:
    perf_stats = {}

    sys.stdout.write(f"{func_name}(...)\n")
    for lang, impl in IMPLEMENTATIONS:
        start = time.time()
        test_failed = False
        for func_args, expected_result in args_expected_results:
            func = getattr(impl, func_name)
            for _ in range(iterations):
                func(*func_args)
            result = func(*func_args)
            if result != expected_result:
                if not test_failed:
                    sys.stdout.write(
                        f"FAILED! ({lang})"
                        f" {func_name}{pprint.pformat(func_args)}"
                        f" returned '{result}'"
                        f" instead of '{expected_result}'\n"
                    )
                    EXITCODE = 1
                    test_failed = True
        end = time.time()
        ms = (end - start) * 1000
        sys.stdout.write(f"  {lang}: {ms} ms\n")

        perf_stats[lang] = ms

    # Compare performance
    #
    # NOTE: We use `max` to avoid division by zero
    x_faster = perf_stats["Python"] / max(perf_stats["Rust"], 0.0000001)
    if x_faster <= 1:
        sys.stdout.write(
            f"FAILED!    {perf_stats['Rust'] / perf_stats['Python']}x slower\n"
        )
        EXITCODE = 1
    else:
        sys.stdout.write(f"     {x_faster}x faster\n")

    if max(perf_stats.values()) > TIMEOUT:
        sys.stdout.write(f"FAILED! The test is too long\n")
        EXITCODE = 1

    if max(perf_stats.values()) > WARN_TIMEOUT:
        sys.stdout.write(f"WARNING! The test is too long\n")

    sys.stdout.write("\n")
