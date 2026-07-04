## Plain-R tests for the wickra-shazam R binding (no testthat dependency).
## Mirrors the Rust/Python/Node/Go/C#/Java tests and doubles as the completeness
## guard: it exercises the full public surface (version + new + command).

library(wickrashazam)

spec <- paste0(
  '{"features":[{"kind":"price","field":"close"}],',
  '"window":1,"metric":"euclid"}'
)

candle <- function(time, close) {
  paste0(
    '{"time":', time, ',"open":', close, ',"high":', close,
    ',"low":', close, ',"close":', close, ',"volume":1}'
  )
}

## version
stopifnot(nzchar(wkshzm_version()))

## index a monotone history, then match the current state
shazam <- wkshzm_new(spec)
history <- paste0("[", paste(
  vapply(1:10, function(i) candle(i, 100 + i), character(1)),
  collapse = ","
), "]")
indexed <- wkshzm_command(shazam, paste0('{"cmd":"index","history":', history, '}'))
stopifnot(grepl('"indexed":10', indexed, fixed = TRUE))

raw <- wkshzm_command(
  shazam,
  paste0('{"cmd":"match","current":[', candle(11, 110), '],"k":3}')
)
stopifnot(grepl('"indexed":10', raw, fixed = TRUE))
## The tail of a monotone history is nearest its latest bar (ts 10).
stopifnot(grepl('"ts":10', raw, fixed = TRUE))

## invalid spec raises
stopifnot(inherits(try(wkshzm_new("not json"), silent = TRUE), "try-error"))

## an unknown command is an in-band error, not a hard error
inband <- wkshzm_command(shazam, '{"cmd":"nope"}')
stopifnot(grepl('"ok":false', inband, fixed = TRUE))

cat("wickra-shazam R tests passed\n")
