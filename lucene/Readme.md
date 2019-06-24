= Lucene benchmarks =

Evaluation of movies dataset with lucene for comparision with tantivy.

Create index:

$ ./gradlew run --args "index"

Run evaluation:

$ ./gradlew run --args "evaluate"
