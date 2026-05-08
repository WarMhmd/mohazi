# @uj_project/verify

Native CLI tool to verify and compile `.vis` files.

## Installation

```bash
npm install @uj_project/verify
```

## Usage

```bash
npx verify [OPTIONS]
```

### Options

- `--config <path>` : Path to config file. Default: `./vis.config.json`.
- `--check` : Run validation check on `.vis` files only, do not generate output.
- `--debug` : Enable verbose debug logging.
- `--version` : Show version number.
- `--help` : Show help information.

## Configuration

The tool requires a `vis.config.json` file in your project root (or specified via `--config`).

### Example `vis.config.json`

```json
{
  "input": "./src/vis",
  "languages": {
    "javascript": {
      "enabled": true,
      "output": "./frontend/src/generated"
    },
    "C#": {
      "enabled": true,
      "output": "./backend/src/generated"
    }
  }
}
```

### Configuration Fields

- `input`: Directory containing your `.vis` source files.
- `languages`: Target languages for code generation.
  - `enabled`: Set to `true` to generate code for this language.
  - `output`: Destination directory for the generated source files.

## Example

1. Create a `.vis` file in your input directory (e.g., `./vis/User.vis`).
2. Define your schema in the `.vis` file.
3. Run the compiler:
   ```bash
   npx verify
   ```
4. The generated files will appear in your configured `output` directories.
