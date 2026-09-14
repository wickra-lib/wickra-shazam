/* Cross-language golden parity and operating-mode equivalence, from C.
 *
 * Golden: build the shazam from each committed golden/specs/*.json, index
 * sym-01's history, match its current window with k = 5 and assert the report
 * equals golden/expected/<spec>.json byte-for-byte. The ABI returns the
 * core's compact command output verbatim, so byte equality is the exact
 * cross-language parity check -- the same one Python, Node, Go, C#, Java, R
 * and WASM make. Candle JSON is built from the raw CSV tokens so no number
 * formatting of this test's own can drift.
 *
 * Operating mode: the command protocol offers two ways to attach a label to a
 * historical window -- `label` before `index` is kept on the handle and
 * applied when the index is built, `label` after `index` goes onto the live
 * index -- and both must yield the same match report; re-indexing keeps it.
 * The core pins this in Rust (operating_modes.rs); this checks the boundary
 * six of the ten language reaches cross.
 *
 * C has no directory API that is portable between POSIX and Windows, so the
 * spec list is globbed by CMake at configure time and written into
 * golden_specs.h. That keeps the property the other bindings get from a
 * runtime glob: a spec added to the corpus is covered here without editing
 * this file.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "golden_specs.h" /* GOLDEN_DIR, GOLDEN_SPECS, GOLDEN_SPEC_COUNT */
#include "wickra_shazam.h"

/* Read a whole file. Caller frees. Returns NULL and reports on failure. */
static char *slurp(const char *path) {
    FILE *file = fopen(path, "rb");
    if (!file) {
        fprintf(stderr, "cannot open %s\n", path);
        return NULL;
    }
    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        return NULL;
    }
    long size = ftell(file);
    if (size < 0 || fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)size + 1);
    if (!buf) {
        fclose(file);
        return NULL;
    }
    size_t got = fread(buf, 1, (size_t)size, file);
    fclose(file);
    buf[got] = '\0';
    return buf;
}

/* Trim ASCII whitespace in place and return the start of the trimmed text. */
static char *trim(char *text) {
    while (*text == ' ' || *text == '\n' || *text == '\r' || *text == '\t') {
        text++;
    }
    size_t len = strlen(text);
    while (len > 0) {
        char last = text[len - 1];
        if (last != ' ' && last != '\n' && last != '\r' && last != '\t') {
            break;
        }
        text[--len] = '\0';
    }
    return text;
}

/* A growable string. */
typedef struct {
    char *buf;
    size_t len;
    size_t cap;
} Str;

static int str_push(Str *s, const char *text, size_t n) {
    if (s->len + n + 1 > s->cap) {
        size_t cap = s->cap ? s->cap : 4096;
        while (cap < s->len + n + 1) {
            cap *= 2;
        }
        char *grown = (char *)realloc(s->buf, cap);
        if (!grown) {
            return 0;
        }
        s->buf = grown;
        s->cap = cap;
    }
    memcpy(s->buf + s->len, text, n);
    s->len += n;
    s->buf[s->len] = '\0';
    return 1;
}

static int str_puts(Str *s, const char *text) { return str_push(s, text, strlen(text)); }

/* Apply one read-only command through the two-call length protocol. Caller
 * frees. The hub caches the response of a mutating command between the length
 * call and the delivering call, so index and label run once, not twice. */
static char *run(WickraShazam *shazam, const char *cmd) {
    int32_t len = wickra_shazam_command(shazam, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed with code %d\n", (int)len);
        return NULL;
    }
    char *out = (char *)malloc((size_t)len + 1);
    if (!out) {
        return NULL;
    }
    if (wickra_shazam_command(shazam, cmd, out, (size_t)len + 1) < 0) {
        free(out);
        return NULL;
    }
    return out;
}

/* Build the candle array as JSON text from the CSV's own tokens. Caller frees. */
static char *candles_json(const char *path) {
    char *text = slurp(path);
    if (!text) {
        return NULL;
    }
    Str out = {0};
    int ok = str_puts(&out, "[");
    int first = 1;
    char *cursor = text;
    while (ok && *cursor) {
        char *line = cursor;
        char *nl = strpbrk(cursor, "\r\n");
        if (nl) {
            *nl = '\0';
            cursor = nl + 1;
        } else {
            cursor += strlen(cursor);
        }
        char *cols[6];
        char *field = line;
        int n = 0;
        while (n < 6) {
            char *comma = strchr(field, ',');
            cols[n++] = field;
            if (!comma) {
                break;
            }
            *comma = '\0';
            field = comma + 1;
        }
        if (n < 6 || !*cols[0] || strspn(cols[0], "0123456789") != strlen(cols[0])) {
            continue; /* header, blank or short line */
        }
        if (!first) {
            ok = str_puts(&out, ",");
        }
        first = 0;
        ok = ok && str_puts(&out, "{\"time\":") && str_puts(&out, cols[0]) && str_puts(&out, ",\"open\":") &&
             str_puts(&out, cols[1]) && str_puts(&out, ",\"high\":") && str_puts(&out, cols[2]) &&
             str_puts(&out, ",\"low\":") && str_puts(&out, cols[3]) && str_puts(&out, ",\"close\":") &&
             str_puts(&out, cols[4]) && str_puts(&out, ",\"volume\":") && str_puts(&out, cols[5]) &&
             str_puts(&out, "}");
    }
    free(text);
    if (!ok || !str_puts(&out, "]")) {
        free(out.buf);
        return NULL;
    }
    return out.buf;
}

/* `{"cmd":"label","ts":<top ts of report>,"label":"golden_top"}`. Caller frees. */
static char *label_cmd(const char *report) {
    const char *at = strstr(report, "\"ts\":");
    if (!at) {
        return NULL;
    }
    at += 5;
    size_t digits = strspn(at, "0123456789");
    Str cmd = {0};
    if (!str_puts(&cmd, "{\"cmd\":\"label\",\"ts\":") || !str_push(&cmd, at, digits) ||
        !str_puts(&cmd, ",\"label\":\"golden_top\"}")) {
        free(cmd.buf);
        return NULL;
    }
    return cmd.buf;
}

/* Apply a sequence of commands to a fresh handle and return the reply of the
 * last one. Caller frees. */
static char *drive(const char *spec, const char *const *cmds, size_t count) {
    WickraShazam *shazam = wickra_shazam_new(spec);
    if (!shazam) {
        fprintf(stderr, "invalid spec\n");
        return NULL;
    }
    char *last = NULL;
    for (size_t i = 0; i < count; i++) {
        free(last);
        last = run(shazam, cmds[i]);
        if (!last) {
            break;
        }
    }
    wickra_shazam_free(shazam);
    return last;
}

int main(void) {
    printf("wickra-shazam %s: golden parity + operating modes over %zu spec(s)\n",
           wickra_shazam_version(), (size_t)GOLDEN_SPEC_COUNT);
    if (GOLDEN_SPEC_COUNT == 0) {
        fprintf(stderr, "golden corpus not found\n");
        return 1;
    }
    char *history = candles_json(GOLDEN_DIR "/data/history/sym-01.csv");
    char *current = candles_json(GOLDEN_DIR "/data/current/sym-01.csv");
    if (!history || !current) {
        free(history);
        free(current);
        return 1;
    }
    Str index_cmd = {0};
    Str match_cmd = {0};
    if (!str_puts(&index_cmd, "{\"cmd\":\"index\",\"history\":") || !str_puts(&index_cmd, history) ||
        !str_puts(&index_cmd, "}") || !str_puts(&match_cmd, "{\"cmd\":\"match\",\"current\":") ||
        !str_puts(&match_cmd, current) || !str_puts(&match_cmd, ",\"k\":5}")) {
        return 1;
    }
    free(history);
    free(current);

    int failures = 0;
    for (size_t i = 0; GOLDEN_SPECS[i]; i++) {
        char spec_path[1024];
        char expected_path[1024];
        snprintf(spec_path, sizeof spec_path, "%s/specs/%s", GOLDEN_DIR, GOLDEN_SPECS[i]);
        snprintf(expected_path, sizeof expected_path, "%s/expected/%s", GOLDEN_DIR, GOLDEN_SPECS[i]);
        char *spec = slurp(spec_path);
        char *expected_raw = slurp(expected_path);
        if (!spec || !expected_raw) {
            free(spec);
            free(expected_raw);
            failures++;
            continue;
        }
        char *expected = trim(expected_raw);

        const char *plain_cmds[] = {index_cmd.buf, match_cmd.buf};
        char *unlabelled = drive(spec, plain_cmds, 2);
        char *label = unlabelled ? label_cmd(unlabelled) : NULL;
        char *labelled_before = NULL;
        char *labelled_after = NULL;
        char *reindexed = NULL;
        if (label) {
            const char *before_cmds[] = {label, index_cmd.buf, match_cmd.buf};
            const char *after_cmds[] = {index_cmd.buf, label, match_cmd.buf};
            const char *reindex_cmds[] = {index_cmd.buf, label, index_cmd.buf, match_cmd.buf};
            labelled_before = drive(spec, before_cmds, 3);
            labelled_after = drive(spec, after_cmds, 3);
            reindexed = drive(spec, reindex_cmds, 4);
        }
        if (!unlabelled || !label || !labelled_before || !labelled_after || !reindexed) {
            fprintf(stderr, "%s: command failed\n", GOLDEN_SPECS[i]);
            failures++;
        } else if (strcmp(trim(unlabelled), expected) != 0) {
            fprintf(stderr, "%s: match does not match the blessed report\n", GOLDEN_SPECS[i]);
            failures++;
        } else if (strcmp(trim(labelled_before), trim(labelled_after)) != 0) {
            fprintf(stderr, "%s: a label before index differs from a label after index\n", GOLDEN_SPECS[i]);
            failures++;
        } else if (!strstr(labelled_after, "golden_top") || strcmp(trim(labelled_after), trim(unlabelled)) == 0) {
            fprintf(stderr, "%s: the label did not ride along\n", GOLDEN_SPECS[i]);
            failures++;
        } else if (strcmp(trim(reindexed), trim(labelled_after)) != 0) {
            fprintf(stderr, "%s: re-indexing changed the report\n", GOLDEN_SPECS[i]);
            failures++;
        } else {
            printf("  %s: ok\n", GOLDEN_SPECS[i]);
        }
        free(unlabelled);
        free(label);
        free(labelled_before);
        free(labelled_after);
        free(reindexed);
        free(spec);
        free(expected_raw);
    }
    free(index_cmd.buf);
    free(match_cmd.buf);
    if (failures) {
        fprintf(stderr, "%d spec(s) failed\n", failures);
        return 1;
    }
    printf("ok\n");
    return 0;
}
