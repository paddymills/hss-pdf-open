# HSS PDF Open
HSS Utility to open PDF files with enhanced terminal output

## Binaries
- **vsd**: View Shop Drawings CLI
- **erep**: SigmaNest eReports Launcher

## Features
- 🎨 **Colored terminal output** with emojis
- 🔧 **Flexible verbosity control** using repeated flags (`-v`, `-vv`, `-vvv`)
- 🌐 **Multi-environment support** (Production, QAS, Development)
- 📊 **Standard Rust logging** with `log` crate integration
- 🔢 **Range expansion** support (e.g., `123-125` → `[123, 124, 125]`)

## Download
Available in [releases](https://github.com/paddymills/view-shop-drawings/releases)

## Build from source
```bash
git clone https://github.com/paddymills/hss-pdf-open.git
cd hss-pdf-open
cargo build --release
```

## Install
```bash
git clone https://github.com/paddymills/hss-pdf-open.git
cd hss-pdf-open
cargo install --path .
```

## Usage

### eReports (`erep`)
```bash
# Basic usage
erep [OPTIONS] [PROGRAM_NUMBERS...]

# Examples
erep 123                    # Open program 123
erep 123 456 789           # Open multiple programs
erep 123-125               # Open range: 123, 124, 125
erep 23 25 -e qas          # Open programs in QAS environment
```

#### Options
- **`-v, --verbose...`** - Increase verbosity (use multiple times: `-v`, `-vv`, `-vvv`)
- **`-q, --quiet`** - Suppress all output
- **`-e, --env <ENV>`** - Select environment: `prd` (default), `qas`, `dev`
- **`-h, --help`** - Show help information
- **`-V, --version`** - Show version information

#### Verbosity Levels
- **Default** (no flags): Shows processing status and results
- **`-v`** (INFO): Adds startup/completion messages and search progress  
- **`-vv`** (DEBUG): Adds file path checking details
- **`-vvv+`** (TRACE): Maximum verbosity
- **`-q`** (QUIET): Complete silence

#### Environment Configuration
- **`prd`** (Production): `\\hssfileserv1\Shops\eReports` *(default)*
- **`qas`** (QAS): `\\hssieng\SNDataQas\eReport`
- **`dev`** (Development): `\\hssieng\SNDataDev\eReport`

#### Examples with Output
```bash
# Default verbosity (WARN level)
$ erep 999
📋 Processing: Single(999)
✅ Opened: 999

# Verbose output (INFO level)
$ erep 999 -v
🚀 Starting eReports launcher...
🌐 Environment: Prd (\\hssfileserv1\Shops\eReports)
📋 Processing: Single(999)
   🔍 Searching for: 999
✅ Opened: 999
✨ Complete!

# QAS environment
$ erep 999 --env qas -v
🚀 Starting eReports launcher...
🌐 Environment: Qas (\\hssieng\SNDataQas\eReport)
📋 Processing: Single(999)
   🔍 Searching for: 999
❌ Not found: 999
✨ Complete!

# Quiet mode
$ erep 999 -q
# (no output)
```

### View Shop Drawings (`vsd`)
```bash
vsd <job> [drawing(s)]
```

## Range Support
Both binaries support range expansion:
- `123-125` expands to `[123, 124, 125]`
- Supports intelligent length fixing for sequential numbers
