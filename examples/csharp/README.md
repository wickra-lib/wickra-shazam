# Wickra Shazam examples — C#

Runnable C# examples for the [Wickra Shazam C# binding](../../bindings/csharp). The binding consumes the C ABI
library through P/Invoke, so build it once before running anything:

```bash
cargo build -p wickra-shazam-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Match
```

## The examples

| Example | What it does |
|---------|--------------|
| `Match/Program.cs` | A runnable .NET example: index a history and match the current state. |
