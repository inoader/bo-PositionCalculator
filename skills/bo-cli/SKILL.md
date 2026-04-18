---
name: bo-cli
description: Use the `bo` command-line calculator for Kelly sizing, Polymarket pricing, stock risk sizing, arbitrage/juice, multi-leg portfolio Kelly, correlated scenario Kelly, 2x2 Nash equilibria, and asset-level labels. Trigger when a user asks an agent to calculate betting/trading position size, expected returns, geometric expectation, arbitrage stakes, portfolio Kelly allocation, game equilibrium, or A/L asset level using the installed `@inoader/bo` CLI.
---

# bo CLI

## Core Workflow

Use `bo` when the user wants a numeric result this tool supports. Prefer non-interactive commands and `--json` for automation.

1. Check availability with `bo --version` if unsure.
2. Use `--json` unless the user specifically wants the human-readable CLI output.
3. Inspect `"ok": false` JSON errors and report the invalid input clearly.
4. Use interactive modes only when the user explicitly asks to walk through prompts.

If `bo` is missing, install it with:

```bash
npm install -g @inoader/bo
```

## Command Selection

| Task | Human output | JSON output |
|---|---|---|
| Standard Kelly | `bo <odds> <win_rate%> [capital]` | `bo --json <odds> <win_rate%> [capital]` |
| Asset level | `bo -L <amount>` | `bo --json -L <amount>` |
| Polymarket | `bo -p <market_price%> <your_probability%> [capital]` | `bo --json -p <market_price%> <your_probability%> [capital]` |
| Stock risk sizing | `bo -s <entry> <target> <stop> <win_rate%> [capital]` | `bo --json -s <entry> <target> <stop> <win_rate%> [capital]` |
| Two-way arbitrage | `bo -a <odds1> <odds2> [capital]` | `bo --json -a <odds1> <odds2> [capital]` |
| Multi-way arbitrage | `bo -A <count> <odds1> ... <oddsN> [capital]` | `bo --json -A <count> <odds1> ... <oddsN> [capital]` |
| 2x2 Nash | `bo -n <a11> <a12> <a21> <a22> <b11> <b12> <b21> <b22>` | `bo --json -n <...8 payoffs...>` |
| Portfolio Kelly | `bo -k <count> <odds1> <win1%> ... [capital]` | `bo --json -k <count> <odds1> <win1%> ... [capital]` |
| Mixed-source portfolio | `bo -k <descriptor1> <descriptor2> ... [capital]` | `bo --json -k <descriptor1> ... [capital]` |
| Correlated scenario Kelly | `bo -K <leg_count> <scenario_count> <p1%> <r11%> ... [capital]` | `bo --json -K <...>` |

Interactive equivalents:

```bash
bo
bo -L
bo -p
bo -s
bo -a
bo -A
bo -n
bo -k
bo -K
```

## Input Conventions

- Percent inputs are entered as `0-100`, not decimals. Use `60` for 60%.
- Polymarket market price is also percent/cents: use `60` for 60c.
- Standard odds must be decimal odds greater than `1.0`.
- Capital, amount, prices, and bankroll values must be positive.
- Correlated scenario returns are percentages and may be negative down to `-100`.
- Correlated scenario probabilities should sum to `100`.

## Output Fields

For Kelly modes, prefer these JSON fields:

- `result.arithmetic_expected_return`: arithmetic expected return per unit risk/stake.
- `result.geometric_expected_return.full_kelly`: geometric expectation at full Kelly sizing.
- `result.geometric_expected_return.half_kelly`: geometric expectation at half Kelly sizing.
- `result.geometric_expected_return.quarter_kelly`: geometric expectation at quarter Kelly sizing.
- `result.recommended_fraction`, `risk_fraction`, or `allocations`: use the mode-specific sizing field.
- `sizing`: present when capital is supplied.

`expected_value` is kept as a compatibility alias for `arithmetic_expected_return`; prefer the new field in new work.

For asset level:

- Text output is exactly two lines, for example:

```text
A8.5
L17.7275
```

- JSON includes `a_label`, `l_label`, `a_level`, and `l_level`.

## Examples

Standard Kelly:

```bash
bo --json 2.0 60 10000
```

Asset level:

```bash
bo -L 50000000
```

Polymarket:

```bash
bo --json -p 60 75 1000
```

Stock sizing:

```bash
bo --json -s 100 120 90 60 10000
```

Two-way arbitrage:

```bash
bo --json -a 2.1 2.2 1000
```

Portfolio Kelly with standard legs:

```bash
bo --json -k 2 2.0 60 2.5 55 10000
```

Portfolio Kelly with mixed descriptors:

```bash
bo --json -k std:2.0:60 pm:60:75 stock:100:120:90:60 10000
```

Correlated scenario Kelly:

```bash
bo --json -K 2 2 50 20 -10 50 -10 20 10000
```

Nash equilibrium:

```bash
bo --json -n 3 0 5 1 3 5 0 1
```

## Reporting Results

When answering users:

- Include the command used when reproducibility matters.
- Report percentages as percentages, not raw decimals.
- For Kelly sizing, distinguish arithmetic expectation from geometric expectation.
- For asset level, preserve labels like `A8.5` and `L17.7275`.
- If the command returns an error, explain the input constraint that failed and show a corrected command when obvious.
