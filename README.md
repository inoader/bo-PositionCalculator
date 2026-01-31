# Kelly Calculator

A command-line Kelly Criterion calculator with Polymarket support.

## Features

- Standard Kelly Criterion calculation (odds + win rate)
- Polymarket mode (market price + your probability)
- Interactive and CLI modes
- Capital allocation suggestions (full, half, quarter Kelly)

## Installation

```bash
cargo build --release
```

The binary will be at `target/release/kelly`.

## Usage

### Standard Mode

```bash
# Interactive
./kelly

# CLI
./kelly <odds> <win_rate> [capital]

# Examples
./kelly 2.0 60              # Odds 2.0, 60% win rate
./kelly 2.0 60 10000        # With 10000 capital
```

### Polymarket Mode

```bash
# Interactive
./kelly -p

# CLI
./kelly -p <market_price> <your_probability> [capital]

# Examples
./kelly -p 60 75            # Market price 60c, you think 75%
./kelly -p 60 75 1000       # With 1000 capital
```

## Kelly Criterion Formula

For standard mode:
```
f* = (bp - q) / b
```
where:
- `b` = net odds (odds - 1)
- `p` = probability of winning
- `q` = probability of losing (1 - p)

For Polymarket:
```
f* = (your_probability - market_price) / (1 - market_price)
```

## License

MIT
