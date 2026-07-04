package wickra

import (
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
)

// Cross-language golden: index sym-01's history, match its current window with
// k=5, and assert the response is byte-identical to golden/expected/<spec>.json.
// Candle JSON is built from the raw CSV tokens so no per-language number
// formatting can drift.

func goldenDir(t *testing.T) string {
	t.Helper()
	dir, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	for i := 0; i < 8; i++ {
		g := filepath.Join(dir, "golden")
		if _, err := os.Stat(filepath.Join(g, "specs")); err == nil {
			return g
		}
		dir = filepath.Dir(dir)
	}
	t.Skip("golden fixtures not present")
	return ""
}

func candlesJSON(t *testing.T, path string) string {
	t.Helper()
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	var rows []string
	for _, line := range strings.Split(string(data), "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		c := strings.Split(line, ",")
		if _, err := strconv.ParseInt(c[0], 10, 64); err != nil {
			continue // header
		}
		rows = append(rows, `{"time":`+c[0]+`,"open":`+c[1]+`,"high":`+c[2]+
			`,"low":`+c[3]+`,"close":`+c[4]+`,"volume":`+c[5]+`}`)
	}
	return "[" + strings.Join(rows, ",") + "]"
}

func TestGolden(t *testing.T) {
	g := goldenDir(t)
	history := candlesJSON(t, filepath.Join(g, "data/history/sym-01.csv"))
	current := candlesJSON(t, filepath.Join(g, "data/current/sym-01.csv"))

	specs, err := filepath.Glob(filepath.Join(g, "specs", "*.json"))
	if err != nil {
		t.Fatal(err)
	}
	if len(specs) == 0 {
		t.Fatal("no golden specs found")
	}
	for _, specPath := range specs {
		name := filepath.Base(specPath)
		specJSON, err := os.ReadFile(specPath)
		if err != nil {
			t.Fatal(err)
		}
		expected, err := os.ReadFile(filepath.Join(g, "expected", name))
		if err != nil {
			t.Fatal(err)
		}
		s, err := New(string(specJSON))
		if err != nil {
			t.Fatal(err)
		}
		if _, err := s.Command(`{"cmd":"index","history":` + history + `}`); err != nil {
			t.Fatal(err)
		}
		got, err := s.Command(`{"cmd":"match","current":` + current + `,"k":5}`)
		s.Close()
		if err != nil {
			t.Fatal(err)
		}
		if strings.TrimSpace(got) != strings.TrimSpace(string(expected)) {
			t.Fatalf("golden mismatch for %s:\n got=%s\n exp=%s", name, got, expected)
		}
	}
}
