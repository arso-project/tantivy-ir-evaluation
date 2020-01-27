# Tantivy Information Retrieval Evaluation

As far as we know this is the first Information Retrieval evaluation of [Tantivy](https://github.com/tantivy-search/tantivy), a "full text search engine library" written in rust.

## Datasets

* Movieset

## Evaluation

### Metrics

### Tasks

For the dataset MOVIE search on the title, body and a combination of both fields (fulltext) is evaluated.

### Lucene

As a reference we used lucene.

| Task               | MP@3    | MP@R       | MAP      |
|--------------------|---------|------------|----------|
| MOVIE (Title)      | 0.134   | 0.065      | 0.039    |
| MOVIE (Body)       | 0.434   | 0.241      | **0.250**|
| MOVIE (Fulltext)   | **0.5** | **0.264**  | 0.237    |

### Tantivy

with limit = 1000

| Task               | MP@3    | MP@R       | MAP      |
|--------------------|---------|------------|----------|
| MOVIE (Title)      | 0.1     | 0.066      | 0.039    |
| MOVIE (Body)       | 0.367   | 0.286      | **0.307**|
| MOVIE (Fulltext)   | **0.5** | **0.297**  | 0.299    |

## Conclusion

For MOVIE Lucene and Tantivy return compareable results. Tantivy is even slightly better, although this could be a result of the limit to 1000 documents.
