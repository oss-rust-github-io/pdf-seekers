# PDF Seekers

![](https://img.shields.io/badge/license-MIT-green)
![](https://img.shields.io/badge/Powered%20By-Rust-blue)
![](https://img.shields.io/badge/crates.io-v0.1.0-blue
)

Simple parser and information extractor from PDF documents based on keyword search functionality (powered by Rust)

<p align="center">
<img
  src="/logo/pdf_seeker.png"
  title="PDF-Seeker"
  width="25%"
  height="25%">
</p>

## Key Features:
- Indexing capability on single PDF file or directory containing multiple PDF files
- Search for keywords across multiple PDF files to get relevant information
- Get number of pages in PDF file, the page numbers containing the search term, and surrounding text aroung the search term

## Getting Started:
Visit the [PDF Seeker official repository](https://github.com/oss-rust-github-io/pdf-seekers.git) for more information.

## Usage
`pdf_seekers.exe [OPTIONS] --action ACTION --file-or-directory FILE_OR_DIRECTORY --index-path INDEX_PATH`

**Options:**
- **-a, --action**: Action to be performed [index, search]
- **-f, --file-or-directory**: Provide single PDF file to be searched, or directory path containing multiple PDF files
- **-i, --index-path**: Directory path where all indexed files will be stored
- **-s, --search-term**: Keyword to be searched in PDF files (only required when action=Searching)
- **-h, --help**: Print help
- **-V, --version**: Print version

## Examples

- **Indexing Command**

`$ cargo run -- -a index -i index_dir -f data`

<p align="left">
<img
  src="examples\indexing_example.png"
  title="PDF-Seeker"
  width="35%"
  height="35%">
</p>

- **Search Command**

`$ cargo run -- -a search -i index_dir -f data -s convolutional`

<p align="left">
<img
  src="examples\search_example.png"
  title="PDF-Seeker"
  width="65%"
  height="35%">
</p>