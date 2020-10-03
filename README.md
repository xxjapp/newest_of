# newest_of
Get newest or oldest objects of input files or directories recursively by modification time

## Usage

```bash
# newest_of --help
newest_of 1.0.0
Get newest or oldest objects of input files or directories recursively by modification time

USAGE:
    newest_of [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                Prints help information
    -d, --output-directory    Output directories
    -r, --reverse             Instead of search newest, search oldest
    -u, --unordered           Do not sort output by modification time, count and reverse will be ignored
    -V, --version             Prints version information

OPTIONS:
    -c, --count <count>                     The max result file/directory count [default: 10]
    -E, --exclude-exts <exclude-exts>...    The extensions to exclude for files
    -e, --include-exts <include-exts>...    The extensions to include for files
    -i, --input-paths <input-paths>...      The input files or directories paths to search [default: ./]
```
