#!/usr/bin/env bash
set -euo pipefail

rustc --edition=2021 --test tests/custom/core_logic_tests.rs -o /tmp/polindora-core-logic-tests
rustc --edition=2021 --test tests/custom/theme_tests.rs -o /tmp/polindora-theme-tests

/tmp/polindora-core-logic-tests
/tmp/polindora-theme-tests
