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

Releases happen quite often (weekly / every few days) at the moment, so updating polars regularly to get the latest bugfixes / features might not be a bad idea.

### Usage Examples

```
>>> import pypdf_seekers as ps
>>>
>>> data_dir = "data"
>>> cache_dir = None
>>> display_logs = True
>>> search_term = "convolutional"
>>>
>>> ps.indexing_contents(data_dir, cache_dir, display_logs)
[2023-11-27 11:18:35.658837100 UTC]     [INFO]  ==================================================
[2023-11-27 11:18:35.659590900 UTC]     [INFO]  Indexing Operation Logs
[2023-11-27 11:18:35.660091400 UTC]     [INFO]  ==================================================
[2023-11-27 11:18:35.660560100 UTC]     [INFO]  Input Parameters:
[2023-11-27 11:18:35.661108900 UTC]     [INFO]  file_or_directory: data
[2023-11-27 11:18:35.661607500 UTC]     [INFO]  cache_path: None
[2023-11-27 11:18:35.662259300 UTC]     [INFO]  display_logs: Some(true)
[2023-11-27 11:18:35.662787600 UTC]     [DEBUG]         data - Directory Flag is true
[2023-11-27 11:18:35.663926500 UTC]     [INFO]  data - Read all file names successfully in directory true
[2023-11-27 11:18:35.664591600 UTC]     [INFO]  data/fast_rcnn.pdf - Indexing started...
[2023-11-27 11:18:35.712787300 UTC]     [INFO]  data/fast_rcnn.pdf - File read successfully
[2023-11-27 11:18:35.713847500 UTC]     [DEBUG]         Index directory created successfully at D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:18:35.714683300 UTC]     [DEBUG]         D:\github-repos\pdf-seekers/cache/index_dir - Is Index directory empty? false
[2023-11-27 11:18:35.716888900 UTC]     [DEBUG]         Read contents successfully of Index directory D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:18:35.718133900 UTC]     [INFO]  Index writer created successfully for D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:18:35.772777500 UTC]     [INFO]  data/fast_rcnn.pdf - Indexing completed.
[2023-11-27 11:18:35.773459300 UTC]     [INFO]  data/yolo.pdf - Indexing started...
[2023-11-27 11:18:35.822782300 UTC]     [INFO]  data/yolo.pdf - File read successfully
[2023-11-27 11:18:35.823806500 UTC]     [DEBUG]         Index directory created successfully at D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:18:35.824933100 UTC]     [DEBUG]         D:\github-repos\pdf-seekers/cache/index_dir - Is Index directory empty? false
[2023-11-27 11:18:35.826418200 UTC]     [DEBUG]         Read contents successfully of Index directory D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:18:35.827264600 UTC]     [INFO]  Index writer created successfully for D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:18:35.875733 UTC]        [INFO]  data/yolo.pdf - Indexing completed.
>>>
>>> docs = ps.search_term_in_file(data_dir, search_term, cache_dir, display_logs)
[2023-11-27 11:19:14.143293100 UTC]     [INFO]  ==================================================
[2023-11-27 11:19:14.144011600 UTC]     [INFO]  Keyword Search Operation Logs
[2023-11-27 11:19:14.144693200 UTC]     [INFO]  ==================================================
[2023-11-27 11:19:14.145358700 UTC]     [INFO]  Input Parameters:
[2023-11-27 11:19:14.145921200 UTC]     [INFO]  file_or_directory: data
[2023-11-27 11:19:14.146519700 UTC]     [INFO]  search_term: convolutional
[2023-11-27 11:19:14.147070700 UTC]     [INFO]  cache_path: None
[2023-11-27 11:19:14.147639700 UTC]     [INFO]  display_logs: Some(true)
[2023-11-27 11:19:14.148478500 UTC]     [DEBUG]         data - Directory Flag is true
[2023-11-27 11:19:14.149539600 UTC]     [DEBUG]         Index directory created successfully at D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:19:14.150629600 UTC]     [DEBUG]         D:\github-repos\pdf-seekers/cache/index_dir - Is Index directory empty? false
[2023-11-27 11:19:14.152049100 UTC]     [DEBUG]         Read contents successfully of Index directory D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:19:14.152895900 UTC]     [INFO]  Index writer created successfully for D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:19:14.160137800 UTC]     [INFO]  Retrieved matched documents successfully for `convolutional` search term
[2023-11-27 11:19:14.209136500 UTC]     [INFO]  data/fast_rcnn.pdf: Metadata extracted successfully
[2023-11-27 11:19:14.257070100 UTC]     [INFO]  data/yolo.pdf: Metadata extracted successfully
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
[Page: 9] 2009.  2 [5]  E. Denton, W. Zaremba, J. Bruna, Y. LeCun, and R. Fergus. Exploiting linear structure within convolutional networks for efcient evaluation. In NIPS , 2014.  4 [6]  D. Erhan, C. Szegedy, A. Toshev, and D.
==================================================
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
`pdf_seekers = "0.1.3"`

### Usage Examples

`cargo run -- --action ACTION --file-or-directory FILE_OR_DIRECTORY`

**Options:**
- **-a, --action**: Action to be performed [index, search]
- **-f, --file-or-directory**: Provide single PDF file to be searched, or directory path containing multiple PDF files
- **-s, --search-term**: Keyword to be searched in PDF files (only required when action=Searching)
- **--cache-path**: Directory path where all indexed files, log files, and tracker files will be stored If no value is provided, then this will be created in current working directory
- **--display-logs**: Flag to indicate whether to display processing logs on screen or not Default value is set to False [possible values: true, false]
- **-h, --help**: Print help
- **-V, --version**: Print version


**Indexing Command**

```
$ cargo run -- -a index -f data --display-logs true
[2023-11-27 11:22:09.531701300 UTC]     [INFO]  ==================================================
[2023-11-27 11:22:09.532237 UTC]        [INFO]  Indexing Operation Logs
[2023-11-27 11:22:09.532559 UTC]        [INFO]  ==================================================
[2023-11-27 11:22:09.532835800 UTC]     [INFO]  Input Parameters:
[2023-11-27 11:22:09.533236800 UTC]     [INFO]  file_or_directory: data
[2023-11-27 11:22:09.533648900 UTC]     [INFO]  cache_path: None
[2023-11-27 11:22:09.534013900 UTC]     [INFO]  display_logs: Some(true)
[2023-11-27 11:22:09.534456200 UTC]     [DEBUG]         data - Directory Flag is true
[2023-11-27 11:22:09.534925100 UTC]     [INFO]  data - Read all file names successfully in directory true
[2023-11-27 11:22:09.535228800 UTC]     [INFO]  data/fast_rcnn.pdf - Indexing started...
[2023-11-27 11:22:09.984656400 UTC]     [INFO]  data/fast_rcnn.pdf - File read successfully
[2023-11-27 11:22:09.985397900 UTC]     [DEBUG]         Index directory created successfully at D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:22:09.985906200 UTC]     [DEBUG]         D:\github-repos\pdf-seekers/cache/index_dir - Is Index directory empty? false
[2023-11-27 11:22:09.989165400 UTC]     [DEBUG]         Read contents successfully of Index directory D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:22:09.989865600 UTC]     [INFO]  Index writer created successfully for D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:22:10.183223700 UTC]     [INFO]  data/fast_rcnn.pdf - Indexing completed.
[2023-11-27 11:22:10.184010900 UTC]     [INFO]  data/yolo.pdf - Indexing started...
[2023-11-27 11:22:10.566968300 UTC]     [INFO]  data/yolo.pdf - File read successfully
[2023-11-27 11:22:10.568236300 UTC]     [DEBUG]         Index directory created successfully at D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:22:10.569197800 UTC]     [DEBUG]         D:\github-repos\pdf-seekers/cache/index_dir - Is Index directory empty? false
[2023-11-27 11:22:10.571250600 UTC]     [DEBUG]         Read contents successfully of Index directory D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:22:10.571887100 UTC]     [INFO]  Index writer created successfully for D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:22:10.755110700 UTC]     [INFO]  data/yolo.pdf - Indexing completed.
```

**Search Command**

```
$ cargo run -- -a search -f data -s convolutional --display-logs true
[2023-11-27 11:24:52.491396600 UTC]     [INFO]  ==================================================
[2023-11-27 11:24:52.492260800 UTC]     [INFO]  Keyword Search Operation Logs
[2023-11-27 11:24:52.492854500 UTC]     [INFO]  ==================================================
[2023-11-27 11:24:52.493425600 UTC]     [INFO]  Input Parameters:
[2023-11-27 11:24:52.493961500 UTC]     [INFO]  file_or_directory: data
[2023-11-27 11:24:52.494504100 UTC]     [INFO]  search_term: convolutional
[2023-11-27 11:24:52.495097500 UTC]     [INFO]  cache_path: None
[2023-11-27 11:24:52.495650800 UTC]     [INFO]  display_logs: Some(true)
[2023-11-27 11:24:52.496248400 UTC]     [DEBUG]         data - Directory Flag is true
[2023-11-27 11:24:52.496901 UTC]        [DEBUG]         Index directory created successfully at D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:24:52.497809900 UTC]     [DEBUG]         D:\github-repos\pdf-seekers/cache/index_dir - Is Index directory empty? false
[2023-11-27 11:24:52.502331900 UTC]     [DEBUG]         Read contents successfully of Index directory D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:24:52.503306600 UTC]     [INFO]  Index writer created successfully for D:\github-repos\pdf-seekers/cache/index_dir
[2023-11-27 11:24:52.529799600 UTC]     [INFO]  Retrieved matched documents successfully for `convolutional` search term
[2023-11-27 11:24:52.986073200 UTC]     [INFO]  data/fast_rcnn.pdf: Metadata extracted successfully
[2023-11-27 11:24:53.404044600 UTC]     [INFO]  data/yolo.pdf: Metadata extracted successfully
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