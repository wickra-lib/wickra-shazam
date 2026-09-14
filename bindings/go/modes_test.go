package wickra

// Operating-mode equivalence through the binding, over the golden corpus. The
// command protocol offers two ways to attach a label to a historical window,
// and both must yield the same `match` report bytes: a `label` sent before
// `index` is kept on the handle and applied when the index is built; a `label`
// sent after `index` goes onto the live index. Re-indexing the same history
// keeps the labels and the report. The core pins this in Rust
// (operating_modes.rs); this checks the boundary the Go binding crosses. A
// missing corpus is a failure, not a skip.

import (
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"testing"
)

const modesLabel = "golden_top"

var topTS = regexp.MustCompile(`"ts":(\d+)`)

func run(t *testing.T, s *Shazam, cmd string) string {
	t.Helper()
	out, err := s.Command(cmd)
	if err != nil {
		t.Fatal(err)
	}
	return strings.TrimSpace(out)
}

func TestALabelBeforeIndexMatchesALabelAfterIndex(t *testing.T) {
	g := goldenDir(t)
	history := candlesJSON(t, filepath.Join(g, "data/history/sym-01.csv"))
	current := candlesJSON(t, filepath.Join(g, "data/current/sym-01.csv"))
	indexCmd := `{"cmd":"index","history":` + history + `}`
	matchCmd := `{"cmd":"match","current":` + current + `,"k":5}`

	specs, err := filepath.Glob(filepath.Join(g, "specs", "*.json"))
	if err != nil || len(specs) == 0 {
		t.Fatal("golden corpus not found")
	}
	sort.Strings(specs)
	for _, specPath := range specs {
		name := filepath.Base(specPath)
		spec, err := os.ReadFile(specPath)
		if err != nil {
			t.Fatal(err)
		}
		expected, err := os.ReadFile(filepath.Join(g, "expected", name))
		if err != nil {
			t.Fatal(err)
		}

		plain, err := New(string(spec))
		if err != nil {
			t.Fatalf("%s: %v", name, err)
		}
		run(t, plain, indexCmd)
		unlabelled := run(t, plain, matchCmd)
		plain.Close()
		if unlabelled != strings.TrimSpace(string(expected)) {
			t.Fatalf("%s: match does not match the blessed report", name)
		}
		ts := topTS.FindStringSubmatch(unlabelled)
		if ts == nil {
			t.Fatalf("%s: no match in the report", name)
		}
		labelCmd := `{"cmd":"label","ts":` + ts[1] + `,"label":"` + modesLabel + `"}`

		before, err := New(string(spec))
		if err != nil {
			t.Fatalf("%s: %v", name, err)
		}
		run(t, before, labelCmd)
		run(t, before, indexCmd)
		labelledBefore := run(t, before, matchCmd)
		before.Close()

		after, err := New(string(spec))
		if err != nil {
			t.Fatalf("%s: %v", name, err)
		}
		run(t, after, indexCmd)
		run(t, after, labelCmd)
		labelledAfter := run(t, after, matchCmd)
		if labelledBefore != labelledAfter {
			t.Fatalf("%s: a label before index differs from a label after index", name)
		}
		if !strings.Contains(labelledAfter, modesLabel) {
			t.Fatalf("%s: the label did not ride along", name)
		}
		if labelledAfter == unlabelled {
			t.Fatalf("%s: the label changed nothing", name)
		}
		run(t, after, indexCmd)
		if run(t, after, matchCmd) != labelledAfter {
			t.Fatalf("%s: re-indexing changed the report", name)
		}
		after.Close()
	}
}
