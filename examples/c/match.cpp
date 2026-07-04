// A minimal C++ example: index a history and match the current state through the
// wickra-shazam C ABI.
#include <cstddef>
#include <iostream>
#include <string>
#include <vector>

#include "wickra_shazam.h"

namespace {
const char *SPEC =
    R"({"features":[{"kind":"price","field":"close"}],)"
    R"("window":1,"metric":"euclid"})";

const char *INDEX =
    R"({"cmd":"index","history":[)"
    R"({"time":1,"open":100,"high":100,"low":100,"close":100,"volume":1},)"
    R"({"time":2,"open":101,"high":101,"low":101,"close":101,"volume":1},)"
    R"({"time":3,"open":102,"high":102,"low":102,"close":102,"volume":1}]})";

const char *MATCH =
    R"({"cmd":"match","current":[)"
    R"({"time":4,"open":102,"high":102,"low":102,"close":102,"volume":1}],)"
    R"("k":2})";

// Run a command with the length-out protocol; returns the response, or an empty
// optional on error.
bool run(WickraShazam *shazam, const char *cmd, std::string &out) {
    int len = wickra_shazam_command(shazam, cmd, nullptr, 0);
    if (len < 0) {
        return false;
    }
    std::vector<char> buf(static_cast<std::size_t>(len) + 1);
    wickra_shazam_command(shazam, cmd, buf.data(),
                          static_cast<std::size_t>(buf.size()));
    out.assign(buf.data());
    return true;
}
}  // namespace

int main() {
    WickraShazam *shazam = wickra_shazam_new(SPEC);
    if (shazam == nullptr) {
        std::cerr << "failed to build shazam\n";
        return 1;
    }

    std::string indexed;
    std::string report;
    if (!run(shazam, INDEX, indexed) || !run(shazam, MATCH, report)) {
        std::cerr << "command failed\n";
        wickra_shazam_free(shazam);
        return 1;
    }

    std::cout << "wickra-shazam " << wickra_shazam_version() << "\n";
    std::cout << "indexed: " << indexed << "\n";
    std::cout << "match: " << report << "\n";

    wickra_shazam_free(shazam);
    return 0;
}
