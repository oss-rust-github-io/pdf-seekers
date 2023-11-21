# PDF Seekers

![](https://img.shields.io/badge/license-MIT-green)
![](https://img.shields.io/badge/Powered%20By-Rust-blue)
![](https://img.shields.io/badge/crates.io-v0.1.0-blue
)
![](https://img.shields.io/badge/pypi-v0.1.1-blue
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
>>> index_dir = "index"
>>> search_term = "convolutional"
>>> 
>>> ps.indexing_contents(data_dir, index_dir)
[2023-11-21 17:28:09.638890800 UTC] [INFO] data/fast_rcnn.pdf - Indexing completed.
Unicode mismatch true f_i "fi" Ok("ﬁ") [64257]
[2023-11-21 17:28:10.047559300 UTC] [INFO] data/yolo.pdf - Indexing completed.
>>> 
>>> docs = ps.search_term_in_file(data_dir, index_dir, search_term)
[2023-11-21 17:28:38.615366 UTC] [INFO] File Name: data/yolo.pdf
[2023-11-21 17:28:38.659109800 UTC] [INFO] File Name: data/fast_rcnn.pdf
>>> 
>>> for doc in docs:
...     doc.show()
...
[2023-11-21 17:28:54.891329600 UTC] [INFO] Number of pages: 10
[2023-11-21 17:28:54.891675200 UTC] [INFO] Search Term:
[2023-11-21 17:28:54.891952800 UTC] [INFO] Page: 1
[2023-11-21 17:28:54.892267200 UTC] [INFO] Extracted Text: is simple and straightforward. Our system (1) resizes the input image to 448  448 , (2) runs a single convolutional net- work on the image, and (3) thresholds the resulting detections by the model’s condence. methods to rst generate potential
[2023-11-21 17:28:54.892822700 UTC] [INFO] Page: 2
[2023-11-21 17:28:54.893112500 UTC] [INFO] Extracted Text: Our nal prediction is a 7  7  30 tensor. 2.1. Network Design We implement this model as a convolutional neural net- work and evaluate it on the P ASCAL VOC detection dataset [ 9 ]. The initial convolutional layers
[2023-11-21 17:28:54.893647900 UTC] [INFO] Page: 3
[2023-11-21 17:28:54.893911700 UTC] [INFO] Extracted Text: Figure 3: The Architecture. Our detection network has 24 convolutional layers followed by 2 fully connected layers. Alternating 1  1 convolutional layers reduce the features space from preceding layers.
[2023-11-21 17:28:54.894457600 UTC] [INFO] Page: 4
[2023-11-21 17:28:54.894790700 UTC] [INFO] Extracted Text: a set of robust features from input images (Haar [ 25 ], SIFT [ 23 ], HOG [ 4 ], convolutional features [ 6 ]). Then, classiers [ 36 ,  21 ,  13 ,  10 ] or localizers
[2023-11-21 17:28:54.895406600 UTC] [INFO] Page: 5
[2023-11-21 17:28:54.895713500 UTC] [INFO] Extracted Text: 14 ]. YOLO shares some similarities with R-CNN. Each grid cell proposes potential bounding boxes and scores those boxes using convolutional features. However, our system puts spatial constraints on the grid cell proposals which helps mitigate multiple detections of the same
[2023-11-21 17:28:54.896272 UTC] [INFO] Page: 9
[2023-11-21 17:28:54.896602900 UTC] [INFO] Extracted Text: [6]  J. Donahue, Y. Jia, O. Vinyals, J. Hoffman, N. Zhang, E. Tzeng, and T. Darrell. Decaf: A deep convolutional acti- vation feature for generic visual recognition. arXiv preprint arXiv:1310.1531 , 2013.  4 [7]  J. Dong, Q. Chen,
[2023-11-21 17:28:54.897219100 UTC] [INFO] Number of pages: 9
[2023-11-21 17:28:54.897568200 UTC] [INFO] Search Term:
[2023-11-21 17:28:54.897864500 UTC] [INFO] Page: 1
[2023-11-21 17:28:54.898179 UTC] [INFO] Extracted Text: method (Fast R-CNN) for object detection. Fast R-CNN builds on previous work to efciently classify ob- ject proposals using deep convolutional networks. Com- pared to previous work, Fast R-CNN employs several in- novations to improve training and testing speed while also
[2023-11-21 17:28:54.898804400 UTC] [INFO] Page: 2
[2023-11-21 17:28:54.899129 UTC] [INFO] Extracted Text: are also written to disk. But unlike R-CNN, the ne-tuning al- gorithm proposed in [ 11 ] cannot update the convolutional layers that precede the spatial pyramid pooling. Unsurpris- ingly, this limitation (xed convolutional layers) limits the accuracy of very deep
[2023-11-21 17:28:54.899753900 UTC] [INFO] Page: 9
[2023-11-21 17:28:54.900076400 UTC] [INFO] Extracted Text: 2009.  2 [5]  E. Denton, W. Zaremba, J. Bruna, Y. LeCun, and R. Fergus. Exploiting linear structure within convolutional networks for efcient evaluation. In NIPS , 2014.  4 [6]  D. Erhan, C. Szegedy, A. Toshev, and D.
```

## Rust

You can take latest release from crates.io, or if you want to use the latest features / performance improvements point to the main branch of this repo.

Run the following Cargo command in your project directory:
`cargo add pdf_seekers`

Or add the following line to your Cargo.toml:
`pdf_seekers = "0.1.1"`

### Usage Examples

`cargo run -- --action ACTION --file-or-directory FILE_OR_DIRECTORY --index-path INDEX_PATH`

**Options:**
- **-a, --action**: Action to be performed [index, search]
- **-f, --file-or-directory**: Provide single PDF file to be searched, or directory path containing multiple PDF files
- **-i, --index-path**: Directory path where all indexed files will be stored
- **-s, --search-term**: Keyword to be searched in PDF files (only required when action=Searching)
- **-h, --help**: Print help
- **-V, --version**: Print version


**Indexing Command**

```
$ cargo run -- -a index -i index_dir -f data
[2023-11-21 17:34:10.144023300 UTC] [INFO] data/fast_rcnn.pdf - Indexing completed.
Unicode mismatch true f_i "fi" Ok("ﬁ") [64257]
[2023-11-21 17:34:11.833275200 UTC] [INFO] data/yolo.pdf - Indexing completed.
```

**Search Command**

```
$ cargo run -- -a search -i index_dir -f data -s convolutional
[2023-11-21 17:34:20.866758200 UTC] [INFO] File Name: data/yolo.pdf
[2023-11-21 17:34:21.343954600 UTC] [INFO] File Name: data/fast_rcnn.pdf
[2023-11-21 17:34:21.344466100 UTC] [INFO] Number of pages: 10
[2023-11-21 17:34:21.344723100 UTC] [INFO] Search Term:
[2023-11-21 17:34:21.344943300 UTC] [INFO] Page: 1
[2023-11-21 17:34:21.345155900 UTC] [INFO] Extracted Text: is simple and straightforward. Our system (1) resizes the input image to 448  448 , (2) runs a single convolutional net- work on the image, and (3) thresholds the resulting detections by the model’s condence. methods to rst generate potential
[2023-11-21 17:34:21.345686100 UTC] [INFO] Page: 2
[2023-11-21 17:34:21.346109200 UTC] [INFO] Extracted Text: Our nal prediction is a 7  7  30 tensor. 2.1. Network Design We implement this model as a convolutional neural net- work and evaluate it on the P ASCAL VOC detection dataset [ 9 ]. The initial convolutional layers
[2023-11-21 17:34:21.346702900 UTC] [INFO] Page: 3
[2023-11-21 17:34:21.347033100 UTC] [INFO] Extracted Text: Figure 3: The Architecture. Our detection network has 24 convolutional layers followed by 2 fully connected layers. Alternating 1  1 convolutional layers reduce the features space from preceding layers.
[2023-11-21 17:34:21.347782300 UTC] [INFO] Page: 4
[2023-11-21 17:34:21.348105600 UTC] [INFO] Extracted Text: a set of robust features from input images (Haar [ 25 ], SIFT [ 23 ], HOG [ 4 ], convolutional features [ 6 ]). Then, classiers [ 36 ,  21 ,  13 ,  10 ] or localizers
[2023-11-21 17:34:21.348597800 UTC] [INFO] Page: 5
[2023-11-21 17:34:21.348833700 UTC] [INFO] Extracted Text: 14 ]. YOLO shares some similarities with R-CNN. Each grid cell proposes potential bounding boxes and scores those boxes using convolutional features. However, our system puts spatial constraints on the grid cell proposals which helps mitigate multiple detections of the same
[2023-11-21 17:34:21.349386700 UTC] [INFO] Page: 9
[2023-11-21 17:34:21.349679500 UTC] [INFO] Extracted Text: [6]  J. Donahue, Y. Jia, O. Vinyals, J. Hoffman, N. Zhang, E. Tzeng, and T. Darrell. Decaf: A deep convolutional acti- vation feature for generic visual recognition. arXiv preprint arXiv:1310.1531 , 2013.  4 [7]  J. Dong, Q. Chen,
[2023-11-21 17:34:21.350348900 UTC] [INFO] Number of pages: 9
[2023-11-21 17:34:21.350626500 UTC] [INFO] Search Term:
[2023-11-21 17:34:21.350912500 UTC] [INFO] Page: 1
[2023-11-21 17:34:21.351197200 UTC] [INFO] Extracted Text: method (Fast R-CNN) for object detection. Fast R-CNN builds on previous work to efciently classify ob- ject proposals using deep convolutional networks. Com- pared to previous work, Fast R-CNN employs several in- novations to improve training and testing speed while also
[2023-11-21 17:34:21.352050100 UTC] [INFO] Page: 2
[2023-11-21 17:34:21.352867100 UTC] [INFO] Extracted Text: are also written to disk. But unlike R-CNN, the ne-tuning al- gorithm proposed in [ 11 ] cannot update the convolutional layers that precede the spatial pyramid pooling. Unsurpris- ingly, this limitation (xed convolutional layers) limits the accuracy of very deep
[2023-11-21 17:34:21.353819600 UTC] [INFO] Page: 9
[2023-11-21 17:34:21.354126600 UTC] [INFO] Extracted Text: 2009.  2 [5]  E. Denton, W. Zaremba, J. Bruna, Y. LeCun, and R. Fergus. Exploiting linear structure within convolutional networks for efcient evaluation. In NIPS , 2014.  4 [6]  D. Erhan, C. Szegedy, A. Toshev, and D.
```

## Official Repository:
Visit the [PDF Seeker official repository](https://github.com/oss-rust-github-io/pdf-seekers.git) for more information.