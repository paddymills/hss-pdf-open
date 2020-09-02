# HSS PDF Open
HSS Utility to ope PDF files

# Binaries
- vsd: View Shop Drawings CLI
- erep: SigmaNest eReports

## Download
in [releases](https://github.com/paddymills/view-shop-drawings/releases)

## Build from source
```
git clone https://github.com/paddymills/hss-pdf-open.git
cd hss-pdf-open
cargo build
```

## Install
```
git clone https://github.com/paddymills/hss-pdf-open.git
cd hss-pdf-open
cargo install --path .
```

## Usage
```
vsd <job> [drawing(s)]
erep [program number(s)]
```

can accept ranges:
x1-10 -> [x1, x2, ... x9, x10]
