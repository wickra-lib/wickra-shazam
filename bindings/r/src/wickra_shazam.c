/* R .Call glue for the wickra-shazam C ABI hub. */
#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>
#include <stddef.h>
#include "wickra_shazam.h"

/* --- handle lifetime ----------------------------------------------------- */

static void wkshzm_finalize(SEXP ext) {
    WickraShazam *h = (WickraShazam *)R_ExternalPtrAddr(ext);
    if (h) {
        wickra_shazam_free(h);
    }
    R_ClearExternalPtr(ext);
}

static WickraShazam *handle_of(SEXP ext) {
    WickraShazam *h = (WickraShazam *)R_ExternalPtrAddr(ext);
    if (!h) {
        Rf_error("wickra-shazam: handle is closed");
    }
    return h;
}

/* --- exported .Call entries ---------------------------------------------- */

SEXP wkshzm_version(void) {
    return Rf_mkString(wickra_shazam_version());
}

SEXP wkshzm_new(SEXP spec_json) {
    WickraShazam *h = wickra_shazam_new(CHAR(STRING_ELT(spec_json, 0)));
    if (!h) {
        Rf_error("wickra-shazam: invalid spec");
    }
    SEXP ext = PROTECT(R_MakeExternalPtr(h, R_NilValue, R_NilValue));
    R_RegisterCFinalizerEx(ext, wkshzm_finalize, TRUE);
    UNPROTECT(1);
    return ext;
}

SEXP wkshzm_command(SEXP ext, SEXP cmd_json) {
    WickraShazam *h = handle_of(ext);
    const char *cmd = CHAR(STRING_ELT(cmd_json, 0));

    /* Length-out protocol: learn the length, then read into a caller buffer.
       Domain errors come back in-band as {"ok":false,...} JSON, not a negative
       code; only unusable arguments / a caught panic return < 0. */
    int len = wickra_shazam_command(h, cmd, NULL, 0);
    if (len < 0) {
        Rf_error("wickra-shazam: command failed (code %d)", len);
    }
    char *buf = (char *)R_alloc((size_t)len + 1, 1);
    wickra_shazam_command(h, cmd, buf, (size_t)len + 1);
    return Rf_mkString(buf);
}

/* --- registration -------------------------------------------------------- */

static const R_CallMethodDef CallEntries[] = {
    {"wkshzm_version", (DL_FUNC)&wkshzm_version, 0},
    {"wkshzm_new", (DL_FUNC)&wkshzm_new, 1},
    {"wkshzm_command", (DL_FUNC)&wkshzm_command, 2},
    {NULL, NULL, 0}};

void R_init_wickrashazam(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
