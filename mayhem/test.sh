#!/usr/bin/env bash
#
# mayhem/test.sh — RUN the additive cbor known-answer test (cbor-kat) that mayhem/build.sh
# already compiled with the project's normal flags. Asserts encode/decode round-trip AND
# fixed CBOR byte-level output (RFC 7049), so a no-op patch that breaks decode/encode fails.
# Emits a CTRF summary and exits non-zero iff any check failed.
set -uo pipefail
[ -n "${SOURCE_DATE_EPOCH:-}" ] || unset SOURCE_DATE_EPOCH
cd "$SRC"

emit_ctrf() {
  local tool="$1" passed="$2" failed="$3" skipped="${4:-0}" pending="${5:-0}" other="${6:-0}"
  local tests=$(( passed + failed + skipped + pending + other ))
  cat > "${CTRF_REPORT:-$SRC/ctrf-report.json}" <<JSON
{
  "results": {
    "tool": { "name": "$tool" },
    "summary": {
      "tests": $tests,
      "passed": $passed,
      "failed": $failed,
      "pending": $pending,
      "skipped": $skipped,
      "other": $other
    }
  }
}
JSON
  printf 'CTRF {"results":{"tool":{"name":"%s"},"summary":{"tests":%d,"passed":%d,"failed":%d,"pending":%d,"skipped":%d,"other":%d}}}\n' \
    "$tool" "$tests" "$passed" "$failed" "$pending" "$skipped" "$other"
  [ "$failed" -eq 0 ]
}

KAT_BIN=/mayhem/cbor-kat
[ -x "$KAT_BIN" ] || { echo "ERROR: $KAT_BIN missing — build.sh should have built it" >&2; emit_ctrf cbor-kat 0 1; exit 1; }

out="$("$KAT_BIN")"
rc=$?
echo "$out"

# Parse the "CHECKS <n> FAILURES <m>" summary line the KAT binary prints.
line="$(printf '%s\n' "$out" | grep -E '^CHECKS ' | tail -1)"
checks="$(printf '%s' "$line" | awk '{print $2}')"
failures="$(printf '%s' "$line" | awk '{print $4}')"
[ -n "$checks" ] || { echo "ERROR: KAT produced no CHECKS summary" >&2; emit_ctrf cbor-kat 0 1; exit 1; }
if [ "$rc" -ne 0 ] && [ "$failures" -eq 0 ]; then failures=1; fi
passed=$(( checks - failures ))

emit_ctrf cbor-kat "$passed" "$failures"
