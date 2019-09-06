# Tantivy Information Retrieval Evaluation

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
| MOVIE (Body)       | 0.434   | 0.241      | **0.250** |
| MOVIE (Fulltext)   | **0.5** | **0.264**  | 0.237    |

### Tantivy

| Task               | MP@3    | MP@R       | MAP      |
|--------------------|---------|------------|----------|
| MOVIE (Fulltext)   | 0.5     | 0.293      | 0.283    |
