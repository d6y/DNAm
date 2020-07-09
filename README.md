# DNA-m Age Calculation

Running existing models to estimate age from CpG methylation samples.

# Example run

With [cargo](https://www.rust-lang.org/learn/get-started) installed, compile and run:

```
$ cargo run -- examples/GSM2122878-57726.csv
   Compiling dnam v0.1.0 (DNAm)
    Finished dev [unoptimized + debuginfo] target(s) in 1.73s
     Running `target/debug/dnam examples/GSM2122878-57726.csv`
Horvath Clock : 17.73 years
DNAm PhenoAge : 37.79 years
```

Or compile once and then run:

```
$ cargo build --release
$ ./target/release/dnam examples/GSM2122878-57726.csv
Horvath Clock : 17.73 years
DNAm PhenoAge : 37.79 years
```

Notes:

- There is no normalization of the CpG values.
- There's no imupation of missing values (they are treated as zero)

# Sources

## Horvath

- https://www.ncbi.nlm.nih.gov/pubmed/30048243
- Coefficients are in Supplementary dataset 2, but I've taken them from http://clockbio.com/cpg-coefficients/ (the file `CpG-coefficients.csv`)

## Phenoage

- https://www.ncbi.nlm.nih.gov/pubmed/29676998
- Coefficients are in Supplementary dataset 2, but I've taken them from http://clockbio.com/wp-content/uploads/2018/08/coef_pheno.csv




