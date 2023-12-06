## FTS - A Minimal Fulltext Search Engine

> [!IMPORTANT]  
> This repository is strictly for educational purpose and has no plan whatsoever for building production ready application.
> We already have a plathera of great tools for fulltext search like [Elasticsearch](https://elastic.co), [OpenSearch](https://opensearch.org) and so on.
 

### Introduction
FTS (Abbreviation for Full-text search) is a search engine that is able to index key value pairs (`HashMap<String, String>` in rust) and search through the indices to retrive the data that has been indexed.

### Working Mechanism
FTS works based on a few principles that are general to any full text search engine.
For FTS to be able to search data, it must first index data, usually strings. During indexing, it creates a 
[inverted index](https://en.wikipedia.org/wiki/Inverted_index) on which the documents and the tokenized data are stored.

When searching, the search query is first cleaned up, tokenized and the results for each token are generated.
The final result is a HashMap that includes the search results of each token organized by the document id.

### Indexing Process
The indexing process begins with cleaning up the strings.
If we are indexing a `HashMap<String, String>`, the search string is the content of every field in the hashmap combined.
The large string is then sent for semantic analysis where it gets rid of any punctuation and the unicode characters are converted into its equivalent ascii text.
The text is then split along the white spaces to generate words which are cleaned up of the stop words, converted into root words and then stored in the tokens along with the document ids.


### Searching Process
The search query goes through the same semantic analysis as the indexing string.
After the tokens are generated, the results for each token are combined to form the complete result.