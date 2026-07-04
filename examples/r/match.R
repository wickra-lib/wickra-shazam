# A runnable R example: index a history and match the current state.
#
#   cargo build -p wickra-shazam-c --release
#   export WKSHZM_INC="$PWD/bindings/c/include"
#   export WKSHZM_LIB="$PWD/target/release"
#   export LD_LIBRARY_PATH="$WKSHZM_LIB:$LD_LIBRARY_PATH"   # PATH on Windows
#   R CMD INSTALL bindings/r
#   Rscript examples/r/match.R

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

shazam <- wkshzm_new(spec)
history <- paste0(
  "[", candle(1, 100), ",", candle(2, 101), ",", candle(3, 102), "]"
)
wkshzm_command(shazam, paste0('{"cmd":"index","history":', history, "}"))
response <- wkshzm_command(
  shazam, paste0('{"cmd":"match","current":[', candle(4, 102), '],"k":2}')
)

cat("wickra-shazam", wkshzm_version(), "\n")
cat(response, "\n")
