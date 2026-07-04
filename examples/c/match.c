/* A minimal C example: index a history and match the current state through the
   wickra-shazam C ABI. */
#include <stdio.h>
#include <stdlib.h>

#include "wickra_shazam.h"

static const char *SPEC =
    "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}],"
    "\"window\":1,\"metric\":\"euclid\"}";

static const char *INDEX =
    "{\"cmd\":\"index\",\"history\":["
    "{\"time\":1,\"open\":100,\"high\":100,\"low\":100,\"close\":100,\"volume\":1},"
    "{\"time\":2,\"open\":101,\"high\":101,\"low\":101,\"close\":101,\"volume\":1},"
    "{\"time\":3,\"open\":102,\"high\":102,\"low\":102,\"close\":102,\"volume\":1}]}";

static const char *MATCH =
    "{\"cmd\":\"match\",\"current\":["
    "{\"time\":4,\"open\":102,\"high\":102,\"low\":102,\"close\":102,\"volume\":1}],"
    "\"k\":2}";

/* Run a command with the length-out protocol; returns a malloc'd response the
   caller must free, or NULL on error. */
static char *run(WickraShazam *shazam, const char *cmd) {
    int len = wickra_shazam_command(shazam, cmd, NULL, 0);
    if (len < 0) {
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        return NULL;
    }
    wickra_shazam_command(shazam, cmd, buf, (size_t)len + 1);
    return buf;
}

int main(void) {
    WickraShazam *shazam = wickra_shazam_new(SPEC);
    if (!shazam) {
        fprintf(stderr, "failed to build shazam\n");
        return 1;
    }

    char *indexed = run(shazam, INDEX);
    char *report = indexed ? run(shazam, MATCH) : NULL;
    if (!indexed || !report) {
        fprintf(stderr, "command failed\n");
        free(indexed);
        free(report);
        wickra_shazam_free(shazam);
        return 1;
    }

    printf("wickra-shazam %s\n", wickra_shazam_version());
    printf("indexed: %s\n", indexed);
    printf("match: %s\n", report);

    free(indexed);
    free(report);
    wickra_shazam_free(shazam);
    return 0;
}
