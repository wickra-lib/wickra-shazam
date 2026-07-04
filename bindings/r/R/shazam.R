#' The wickra-shazam library version.
#' @return A version string.
#' @export
wkshzm_version <- function() {
  .Call(C_wkshzm_version)
}

#' Build a shazam from a spec JSON string.
#' @param spec_json A JSON spec string.
#' @return A `wickra_shazam` handle (an external pointer).
#' @export
wkshzm_new <- function(spec_json) {
  .Call(C_wkshzm_new, spec_json)
}

#' Apply a command JSON and return the resulting response JSON.
#' @param shazam A shazam handle from [wkshzm_new()].
#' @param cmd_json A command JSON string.
#' @return The response as a JSON string.
#' @export
wkshzm_command <- function(shazam, cmd_json) {
  .Call(C_wkshzm_command, shazam, cmd_json)
}
