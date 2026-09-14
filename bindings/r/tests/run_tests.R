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

## cross-language golden parity: index sym-01's history, match its current
## window with k=5, and assert the response is byte-identical to
## golden/expected/<spec>.json. Candle JSON is built from the raw CSV tokens so
## no per-language number formatting can drift. A missing corpus is a failure,
## not a skip.
golden_dir <- function() {
  d <- normalizePath(getwd(), mustWork = FALSE)
  for (i in seq_len(8)) {
    g <- file.path(d, "golden")
    if (dir.exists(file.path(g, "specs"))) {
      return(g)
    }
    d <- dirname(d)
  }
  NULL
}

candles_json <- function(path) {
  rows <- c()
  for (line in readLines(path, warn = FALSE)) {
    line <- trimws(line)
    if (!nzchar(line)) next
    cc <- strsplit(line, ",")[[1]]
    if (is.na(suppressWarnings(as.integer(cc[1])))) next
    rows <- c(rows, sprintf(
      '{"time":%s,"open":%s,"high":%s,"low":%s,"close":%s,"volume":%s}',
      cc[1], cc[2], cc[3], cc[4], cc[5], cc[6]
    ))
  }
  paste0("[", paste(rows, collapse = ","), "]")
}

g <- golden_dir()
stopifnot(!is.null(g))
history <- candles_json(file.path(g, "data/history/sym-01.csv"))
current <- candles_json(file.path(g, "data/current/sym-01.csv"))
index_cmd <- paste0('{"cmd":"index","history":', history, "}")
match_cmd <- paste0('{"cmd":"match","current":', current, ',"k":5}')
for (spec_path in list.files(file.path(g, "specs"), pattern = "\\.json$", full.names = TRUE)) {
  name <- basename(spec_path)
  spec_json <- paste(readLines(spec_path, warn = FALSE), collapse = "\n")
  expected <- trimws(paste(
    readLines(file.path(g, "expected", name), warn = FALSE), collapse = "\n"
  ))
  gsh <- wkshzm_new(spec_json)
  wkshzm_command(gsh, index_cmd)
  got <- wkshzm_command(gsh, match_cmd)
  stopifnot(identical(trimws(got), expected))
}
cat("wickra-shazam golden parity passed\n")

## Operating-mode equivalence over the golden corpus: a label sent before
## `index` is kept on the handle and applied when the index is built; a label
## sent after `index` goes onto the live index. Both must yield the same match
## report, re-indexing keeps it, and the label rides along on the top match. The
## core pins this in Rust; this checks the boundary the R binding crosses.
label <- "golden_top"
for (spec_path in list.files(file.path(g, "specs"), pattern = "\\.json$", full.names = TRUE)) {
  spec_json <- paste(readLines(spec_path, warn = FALSE), collapse = "\n")
  plain <- wkshzm_new(spec_json)
  wkshzm_command(plain, index_cmd)
  unlabelled <- wkshzm_command(plain, match_cmd)
  ts <- regmatches(unlabelled, regexpr('"ts":[0-9]+', unlabelled))
  ts <- sub('"ts":', "", ts, fixed = TRUE)
  stopifnot(nzchar(ts))
  label_cmd <- paste0('{"cmd":"label","ts":', ts, ',"label":"', label, '"}')

  before <- wkshzm_new(spec_json)
  wkshzm_command(before, label_cmd)
  wkshzm_command(before, index_cmd)
  labelled_before <- wkshzm_command(before, match_cmd)

  after <- wkshzm_new(spec_json)
  wkshzm_command(after, index_cmd)
  wkshzm_command(after, label_cmd)
  labelled_after <- wkshzm_command(after, match_cmd)
  stopifnot(identical(labelled_before, labelled_after))
  stopifnot(grepl(label, labelled_after, fixed = TRUE))
  stopifnot(!identical(labelled_after, unlabelled))

  wkshzm_command(after, index_cmd)
  stopifnot(identical(wkshzm_command(after, match_cmd), labelled_after))
}
cat("wickra-shazam R operating modes: label before index equals label after\n")

cat("wickra-shazam R tests passed\n")
