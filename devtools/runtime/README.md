# E2E Test Runtime

This directory contains tooling to build a custom People Rococo runtime for e2e tests with the following configuration:

- `MaxRingSize = ConstU32<2>`
- `OnboardingQueuePageSize = ConstU32<2>`

## How It Works

The `build-runtime.sh` script in `devtools/scripts/` will:
1. Clone the individuality repository (if not present)
2. Temporarily patch the runtime configuration
3. Build the runtime
4. Copy the compiled WASM to `devtools/runtime/people_rococo_runtime.wasm`
5. Restore the original configuration

## Output

After running the build script, you'll have:
- `people_rococo_runtime.wasm` - The compiled runtime for e2e tests
