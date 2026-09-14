# Shipped with the package and run by `R CMD check`, so it must work inside the
# built tarball -- where no repository sits above it and no fixture file exists.
# It touches nothing but the package: load the library, take a handle, drive one
# command through the boundary, and read the version back.
#
# The cross-language golden parity test lives in run_tests.R, which is excluded
# from the tarball by .Rbuildignore and run from the repository by CI. That one
# needs golden/ above it; this one needs nothing.

library(wickrashazam)

v <- wkshzm_version()
stopifnot(is.character(v), length(v) == 1L, nzchar(v))

h <- wkshzm_new('{"features":[{"kind":"price","field":"close"}],"window":1,"metric":"euclid"}')
out <- wkshzm_command(h, '{"cmd":"version"}')
stopifnot(is.character(out), length(out) == 1L)
stopifnot(grepl("version", out, fixed = TRUE))

cat("wickra-shazam R package smoke: ok (version ", v, ")\n", sep = "")
