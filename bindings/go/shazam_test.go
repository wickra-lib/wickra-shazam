package wickra

import (
	"encoding/json"
	"strings"
	"testing"
)

const spec = `{"features":[{"kind":"price","field":"close"}],"window":1,"metric":"euclid"}`

func candle(time int, close float64) map[string]float64 {
	return map[string]float64{
		"time": float64(time), "open": close, "high": close,
		"low": close, "close": close, "volume": 1,
	}
}

func TestVersion(t *testing.T) {
	if Version() == "" {
		t.Fatal("empty version")
	}
}

func TestIndexAndMatch(t *testing.T) {
	s, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer s.Close()

	history := make([]map[string]float64, 0, 10)
	for i := 1; i <= 10; i++ {
		history = append(history, candle(i, 100+float64(i)))
	}
	idxCmd, err := json.Marshal(map[string]any{"cmd": "index", "history": history})
	if err != nil {
		t.Fatal(err)
	}
	if _, err := s.Command(string(idxCmd)); err != nil {
		t.Fatal(err)
	}

	matchCmd, err := json.Marshal(map[string]any{
		"cmd":     "match",
		"current": []map[string]float64{candle(11, 110)},
		"k":       3,
	})
	if err != nil {
		t.Fatal(err)
	}
	raw, err := s.Command(string(matchCmd))
	if err != nil {
		t.Fatal(err)
	}

	var report struct {
		Indexed int `json:"indexed"`
		Matches []struct {
			Ts int64 `json:"ts"`
		} `json:"matches"`
	}
	if err := json.Unmarshal([]byte(raw), &report); err != nil {
		t.Fatal(err)
	}
	if report.Indexed != 10 {
		t.Fatalf("expected indexed=10, got %d", report.Indexed)
	}
	if len(report.Matches) != 3 || report.Matches[0].Ts != 10 {
		t.Fatalf("expected the nearest match at ts=10, got %+v", report.Matches)
	}
}

func TestInvalidSpec(t *testing.T) {
	if _, err := New("not json"); err == nil {
		t.Fatal("expected an error for an invalid spec")
	}
}

func TestUnknownCommandIsInBandError(t *testing.T) {
	s, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer s.Close()

	// An unknown command is not a hard error: the C ABI returns a length and the
	// error surfaces in-band as {"ok":false,...} JSON.
	raw, err := s.Command(`{"cmd":"nope"}`)
	if err != nil {
		t.Fatalf("unexpected hard error: %v", err)
	}
	if !strings.Contains(raw, `"ok":false`) {
		t.Fatalf("expected an in-band error, got: %s", raw)
	}
}
