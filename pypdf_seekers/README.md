# PDF Seekers

![](https://img.shields.io/badge/license-MIT-green)
![](https://img.shields.io/badge/Powered%20By-Rust-blue)
![](https://img.shields.io/badge/crates.io-v0.1.3-blue
)
![](https://img.shields.io/badge/pypi-v0.1.3-blue
)

Simple parser and information extractor from PDF documents based on keyword search functionality (powered by Rust)

<p align="left">
<img
  src="/logo/pdf_seeker.jpg"
  title="PDF-Seeker"
  width="10%"
  height="10%">
</p>

## Key Features:
- Indexing capability on single PDF file or directory containing multiple PDF files
- Search for keywords across multiple PDF files to get relevant information
- Get number of pages in PDF file, the page numbers containing the search term, and surrounding text aroung the search term

## Python

**Installation**

Install the latest pypdf-seekers version with:
`pip install pypdf-seekers`

Releases happen quite often (weekly / every few days) at the moment, so updating pypdf-seekers regularly to get the latest bugfixes / features might not be a bad idea.

### Usage Examples

```
>>> import pypdf_seekers as ps
>>>
>>> data_dir = "data"
>>> cache_path = None
>>> log_level = "debug"
>>> search_term = "convolutional"
>>>                   
>>> ps.indexing_contents(data_dir, cache_path, log_level)
2023-12-09 14:37:31 | INFO  | Starting indexing operation...
2023-12-09 14:37:31 | DEBUG | src\lib.rs:69 - Input parameters:
2023-12-09 14:37:31 | DEBUG | src\lib.rs:70 - file_or_directory: data
2023-12-09 14:37:31 | DEBUG | src\lib.rs:71 - cache_path: None
2023-12-09 14:37:31 | DEBUG | src\lib.rs:72 - log_level: Some("debug")
2023-12-09 14:37:31 | INFO  | Received `data` which is directory.
2023-12-09 14:37:31 | INFO  | Read all file names successfully in directory `data`
2023-12-09 14:37:31 | INFO  | Read all processed file names successfully in `D:\github-repos\pdf-seekers/.cache/track_dir/_SUCCESS.txt`
2023-12-09 14:37:31 | INFO  | data/fast_rcnn.pdf - Indexing started...
2023-12-09 14:37:31 | INFO  | data/fast_rcnn.pdf - File read successfully.
2023-12-09 14:37:31 | INFO  | Index directory created successfully at `D:\github-repos\pdf-seekers/.cache/index_dir`.
2023-12-09 14:37:31 | DEBUG | src\index_operations.rs:38 - Is index directory `D:\github-repos\pdf-seekers/.cache/index_dir` empty? -> true
2023-12-09 14:37:31 | INFO  | D:\github-repos\pdf-seekers/.cache/index_dir - Directory is empty.
2023-12-09 14:37:31 | INFO  | Index writer created successfully for `D:\github-repos\pdf-seekers/.cache/index_dir` directory.
2023-12-09 14:37:31 | INFO  | data/fast_rcnn.pdf - Indexing completed successfully.
2023-12-09 14:37:31 | INFO  | data/yolo.pdf - Indexing started...
2023-12-09 14:37:31 | INFO  | data/yolo.pdf - File read successfully.
2023-12-09 14:37:31 | INFO  | Index directory created successfully at `D:\github-repos\pdf-seekers/.cache/index_dir`.
2023-12-09 14:37:31 | DEBUG | src\index_operations.rs:38 - Is index directory `D:\github-repos\pdf-seekers/.cache/index_dir` empty? -> false
2023-12-09 14:37:31 | INFO  | D:\github-repos\pdf-seekers/.cache/index_dir - Directory content read successfully.
2023-12-09 14:37:31 | INFO  | Index writer created successfully for `D:\github-repos\pdf-seekers/.cache/index_dir` directory.
2023-12-09 14:37:31 | INFO  | data/yolo.pdf - Indexing completed successfully.
>>>
>>> docs = ps.search_term_in_file(data_dir, search_term, cache_path, log_level)
2023-12-09 14:38:32 | INFO  | Starting searching operation...
2023-12-09 14:38:32 | DEBUG | src\lib.rs:193 - Input parameters:
2023-12-09 14:38:32 | DEBUG | src\lib.rs:194 - file_or_directory: data
2023-12-09 14:38:32 | DEBUG | src\lib.rs:195 - search_term: convolutional
2023-12-09 14:38:32 | DEBUG | src\lib.rs:196 - cache_path: None
2023-12-09 14:38:32 | DEBUG | src\lib.rs:197 - log_level: Some("debug")
2023-12-09 14:38:32 | INFO  | Received `data` which is directory.
2023-12-09 14:38:32 | INFO  | Index directory created successfully at `D:\github-repos\pdf-seekers/.cache/index_dir`.
2023-12-09 14:38:32 | DEBUG | src\index_operations.rs:38 - Is index directory `D:\github-repos\pdf-seekers/.cache/index_dir` empty? -> false
2023-12-09 14:38:32 | INFO  | D:\github-repos\pdf-seekers/.cache/index_dir - Directory content read successfully.
2023-12-09 14:38:32 | INFO  | Index writer created successfully for `D:\github-repos\pdf-seekers/.cache/index_dir`.
2023-12-09 14:38:32 | INFO  | Retrieved matched documents successfully for `convolutional` search term.
2023-12-09 14:38:32 | INFO  | Read all file names successfully in directory `data`
2023-12-09 14:38:32 | INFO  | data/fast_rcnn.pdf: Metadata extracted successfully.
2023-12-09 14:38:32 | INFO  | data/yolo.pdf: Metadata extracted successfully.
>>>
>>> for doc in docs:
...     doc.show()
...
==================================================
Document Name: data/fast_rcnn.pdf
Number of pages: 9
Search Results:
[Page: 1] method (Fast R-CNN) for object detection. Fast R-CNN builds on previous work to efciently classify ob- ject proposals using deep convolutional networks. Com- pared to previous work, Fast R-CNN employs several in- novations to improve training and testing speed while also
[Page: 2] are also written to disk. But unlike R-CNN, the ne-tuning al- gorithm proposed in [ 11 ] cannot update the convolutional layers that precede the spatial pyramid pooling. Unsurpris- ingly, this limitation (xed convolutional layers) limits the accuracy of very deep
[Page: 9] 2009.  2 [5]  E. Denton, W. Zaremba, J. Bruna, Y. LeCun, and R. Fergus. Exploiting linear structure within convolutional networks for efcient evaluation. In NIPS , 2014.  4 [6]  D. Erhan, C. Szegedy, A. Toshev, 
==================================================
Document Name: data/yolo.pdf
Number of pages: 10
Search Results:
[Page: 1] is simple and straightforward. Our system (1) resizes the input image to 448  448 , (2) runs a single convolutional net- work on the image, and (3) thresholds the resulting detections by the model’s condence. methods to rst generate potential
[Page: 2] Our nal prediction is a 7  7  30 tensor. 2.1. Network Design We implement this model as a convolutional neural net- work and evaluate it on the P ASCAL VOC detection dataset [ 9 ]. The initial convolutional layers
[Page: 3] Figure 3: The Architecture. Our detection network has 24 convolutional layers followed by 2 fully connected layers. Alternating 1  1 convolutional layers reduce the features space from preceding layers.
[Page: 4] a set of robust features from input images (Haar [ 25 ], SIFT [ 23 ], HOG [ 4 ], convolutional features [ 6 ]). Then, classiers [ 36 ,  21 ,  13 ,  10 ] or localizers
[Page: 5] 14 ]. YOLO shares some similarities with R-CNN. Each grid cell proposes potential bounding boxes and scores those boxes using convolutional features. However, our system puts spatial constraints on the grid cell proposals which helps mitigate multiple detections of the same
[Page: 9] [6]  J. Donahue, Y. Jia, O. Vinyals, J. Hoffman, N. Zhang, E. Tzeng, and T. Darrell. Decaf: A deep convolutional acti- vation feature for generic visual recognition. arXiv preprint arXiv:1310.1531 , 2013.  4 [7]  J. Dong, Q. Chen,
```

## Rust

You can take latest release from crates.io, or if you want to use the latest features / performance improvements point to the main branch of this repo.

Run the following Cargo command in your project directory:
`cargo add pdf_seekers`

Or add the following line to your Cargo.toml:
`pdf_seekers = "0.1.4"`

### Usage Examples

`cargo run -- --action ACTION --file-or-directory FILE_OR_DIRECTORY`

**Options:**
- **-a, --action**: Action to be performed [index, search]
- **-f, --file-or-directory**: Provide single PDF file to be searched, or directory path containing multiple PDF files
- **-s, --search-term**: Keyword to be searched in PDF files (only required when action=Searching)
- **-c, --cache-path**: Directory path where all indexed files, log files, and tracker files will be stored If no value is provided, then this will be created in current working directory
- **-l, --log-level**: Flag to indicate verbosity level for logs Default value is set to Info. Allowed values are INFO, WARN, DEBUG, ERROR, TRACE, OFF
- **-h, --help**: Print help
- **-V, --version**: Print version


**Indexing Command**

```
$ cargo run -- -a index -f data -l debug
2023-12-09 14:42:00 | INFO  | Starting indexing operation...
2023-12-09 14:42:00 | DEBUG | src\lib.rs:68 - Input parameters:
2023-12-09 14:42:00 | DEBUG | src\lib.rs:69 - file_or_directory: data
2023-12-09 14:42:00 | DEBUG | src\lib.rs:70 - cache_path: None
2023-12-09 14:42:00 | DEBUG | src\lib.rs:71 - log_level: Some("debug")
2023-12-09 14:42:00 | INFO  | Received `data` which is directory.
2023-12-09 14:42:00 | INFO  | Read all file names successfully in directory `data`
2023-12-09 14:42:00 | INFO  | Read all processed file names successfully in `D:\github-repos\pdf-seekers/.cache/track_dir/_SUCCESS.txt`
2023-12-09 14:42:00 | INFO  | data/fast_rcnn.pdf - Indexing started...
2023-12-09 14:42:00 | INFO  | data/fast_rcnn.pdf - File read successfully.
2023-12-09 14:42:00 | INFO  | Index directory created successfully at `D:\github-repos\pdf-seekers/.cache/index_dir`.
2023-12-09 14:42:00 | DEBUG | src\index_operations.rs:38 - Is index directory `D:\github-repos\pdf-seekers/.cache/index_dir` empty? -> true
2023-12-09 14:42:00 | INFO  | D:\github-repos\pdf-seekers/.cache/index_dir - Directory is empty.
2023-12-09 14:42:00 | INFO  | Index writer created successfully for `D:\github-repos\pdf-seekers/.cache/index_dir` directory.
2023-12-09 14:42:01 | INFO  | data/fast_rcnn.pdf - Indexing completed successfully.
2023-12-09 14:42:01 | INFO  | data/yolo.pdf - Indexing started...
2023-12-09 14:42:01 | INFO  | data/yolo.pdf - File read successfully.
2023-12-09 14:42:01 | INFO  | Index directory created successfully at `D:\github-repos\pdf-seekers/.cache/index_dir`.
2023-12-09 14:42:01 | DEBUG | src\index_operations.rs:38 - Is index directory `D:\github-repos\pdf-seekers/.cache/index_dir` empty? -> false
2023-12-09 14:42:01 | INFO  | D:\github-repos\pdf-seekers/.cache/index_dir - Directory content read successfully.
2023-12-09 14:42:01 | INFO  | Index writer created successfully for `D:\github-repos\pdf-seekers/.cache/index_dir` directory.
2023-12-09 14:42:01 | INFO  | data/yolo.pdf - Indexing completed successfully.
```

**Search Command**

```
$ cargo run -- -a search -f data -s convolutional -l debug
2023-12-09 14:42:34 | INFO  | Starting searching operation...
2023-12-09 14:42:34 | DEBUG | src\lib.rs:191 - Input parameters:
2023-12-09 14:42:34 | DEBUG | src\lib.rs:192 - file_or_directory: data
2023-12-09 14:42:34 | DEBUG | src\lib.rs:193 - search_term: convolutional
2023-12-09 14:42:34 | DEBUG | src\lib.rs:194 - cache_path: None
2023-12-09 14:42:34 | DEBUG | src\lib.rs:195 - log_level: Some("debug")
2023-12-09 14:42:34 | INFO  | Received `data` which is directory.
2023-12-09 14:42:34 | INFO  | Index directory created successfully at `D:\github-repos\pdf-seekers/.cache/index_dir`.
2023-12-09 14:42:34 | DEBUG | src\index_operations.rs:38 - Is index directory `D:\github-repos\pdf-seekers/.cache/index_dir` empty? -> false
2023-12-09 14:42:34 | INFO  | D:\github-repos\pdf-seekers/.cache/index_dir - Directory content read successfully.
2023-12-09 14:42:34 | INFO  | Index writer created successfully for `D:\github-repos\pdf-seekers/.cache/index_dir`.
2023-12-09 14:42:34 | DEBUG | src\search_operations.rs:56 - Index reader object created successfully.
2023-12-09 14:42:34 | DEBUG | src\search_operations.rs:60 - Index searcher object created successfully.
2023-12-09 14:42:34 | DEBUG | src\search_operations.rs:68 - Query parser created successfully for `content` field.
2023-12-09 14:42:34 | DEBUG | src\search_operations.rs:75 - Query parsing completed successfully for query string -> convolutional
2023-12-09 14:42:34 | DEBUG | src\search_operations.rs:82 - Top 10 matched documents retrived from search.
2023-12-09 14:42:34 | INFO  | Retrieved matched documents successfully for `convolutional` search term.
2023-12-09 14:42:34 | INFO  | Read all file names successfully in directory `data`
2023-12-09 14:42:34 | INFO  | data/fast_rcnn.pdf: Metadata extracted successfully.
2023-12-09 14:42:35 | INFO  | data/yolo.pdf: Metadata extracted successfully.
==================================================
Document Name: data/fast_rcnn.pdf
Number of pages: 9
Search Results:
[Page: 1] method (Fast R-CNN) for object detection. Fast R-CNN builds on previous work to efciently classify ob- ject proposals using deep convolutional networks. Com- pared to previous work, Fast R-CNN employs several in- novations to improve training and testing speed while also
[Page: 2] are also written to disk. But unlike R-CNN, the ne-tuning al- gorithm proposed in [ 11 ] cannot update the convolutional layers that precede the spatial pyramid pooling. Unsurpris- ingly, this limitation (xed convolutional layers) limits the accuracy of very deep
[Page: 9] 2009.  2 [5]  E. Denton, W. Zaremba, J. Bruna, Y. LeCun, and R. Fergus. Exploiting linear structure within convolutional networks for efcient evaluation. In NIPS , 2014.  4 [6]  D. Erhan, C. Szegedy, A. Toshev, and D.
==================================================
Document Name: data/yolo.pdf
Number of pages: 10
Search Results:
[Page: 1] is simple and straightforward. Our system (1) resizes the input image to 448  448 , (2) runs a single convolutional net- work on the image, and (3) thresholds the resulting detections by the model’s condence. methods to rst generate potential
[Page: 2] Our nal prediction is a 7  7  30 tensor. 2.1. Network Design We implement this model as a convolutional neural net- work and evaluate it on the P ASCAL VOC detection dataset [ 9 ]. The initial convolutional layers
[Page: 3] Figure 3: The Architecture. Our detection network has 24 convolutional layers followed by 2 fully connected layers. Alternating 1  1 convolutional layers reduce the features space from preceding layers.
[Page: 4] a set of robust features from input images (Haar [ 25 ], SIFT [ 23 ], HOG [ 4 ], convolutional features [ 6 ]). Then, classiers [ 36 ,  21 ,  13 ,  10 ] or localizers
[Page: 5] 14 ]. YOLO shares some similarities with R-CNN. Each grid cell proposes potential bounding boxes and scores those boxes using convolutional features. However, our system puts spatial constraints on the grid cell proposals which helps mitigate multiple detections of the same
[Page: 9] [6]  J. Donahue, Y. Jia, O. Vinyals, J. Hoffman, N. Zhang, E. Tzeng, and T. Darrell. Decaf: A deep convolutional acti- vation feature for generic visual recognition. arXiv preprint arXiv:1310.1531 , 2013.  4 [7]  J. Dong, Q. Chen,
```

## Official Repository:
Visit the [PDF Seeker official repository](https://github.com/oss-rust-github-io/pdf-seekers.git) for more information.