// A minimal C++ example: index a history, label a window, match the current
// state -- and show that the label lands the same whether it was sent before
// or after the index was built -- all through the C++ hull.
//
// This goes through `wickra_shazam.hpp`, the C++ hull shipped beside the C
// header, because that hull is what a C++ caller is meant to use: it owns and
// frees the handle, runs the two-call length protocol behind
// `wickra_shazam_command` for you -- the core carries the produced-but-
// undelivered response between the two calls, so a mutating `index` runs once,
// not twice -- and turns a refusal into an exception rather than a negative
// integer that is easy to ignore. Calling the C functions directly from C++
// works too, but then the hull would be shipped without anything building it.
#include <cstdio>
#include <string>

#include "wickra_shazam.hpp"

namespace {
const char *SPEC =
    R"({"features":[{"kind":"price","field":"close"}],)"
    R"("window":1,"metric":"euclid"})";

const char *INDEX =
    R"({"cmd":"index","history":[)"
    R"({"time":1,"open":100,"high":100,"low":100,"close":100,"volume":1},)"
    R"({"time":2,"open":101,"high":101,"low":101,"close":101,"volume":1},)"
    R"({"time":3,"open":102,"high":102,"low":102,"close":102,"volume":1}]})";

const char *LABEL = R"({"cmd":"label","ts":3,"label":"top_of_range"})";

const char *MATCH =
    R"({"cmd":"match","current":[)"
    R"({"time":4,"open":102,"high":102,"low":102,"close":102,"volume":1}],)"
    R"("k":2})";
}  // namespace

int main() {
    try {
        std::printf("wickra-shazam %s\n", wickra::Shazam::version().c_str());

        // Index, then label the window the current state will match.
        wickra::Shazam after(SPEC);
        after.command(INDEX);
        after.command(LABEL);
        const std::string labelled_after = after.command(MATCH);
        std::printf("matches: %s\n", labelled_after.c_str());

        // The same label declared before the index exists: kept on the handle
        // and applied when the index is built.
        wickra::Shazam before(SPEC);
        before.command(LABEL);
        before.command(INDEX);
        const std::string labelled_before = before.command(MATCH);

        // Both orders yield the same report.
        if (labelled_before != labelled_after) {
            std::fprintf(stderr, "a label before index differs from a label after index\n");
            return 1;
        }
    } catch (const wickra::ShazamError &err) {
        // Every failure arrives here: a spec the core rejects, a command it does
        // not understand, a call that returned a negative code.
        std::fprintf(stderr, "%s\n", err.what());
        return 1;
    }
    return 0;
}
