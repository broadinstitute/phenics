# phenics
Phenics is a phenotype simulator for large datasets using additive model.

#Features

* Process multiple VCF files in parallel, then merge liabilities before 
calculating phenotypes.
* Single pass for multiple phenotypes using little memory
* Allele effect is normal for all or some variants
* Normally distributed environmental effect based on given heritability
* Binary phenotype based on given prevalence and given values for case and control

#Workflow

1. Write phenotype definitions, verify using `check`
2. Run `vcf` for each VCF file, producing liabilities file
3. Run `merge` to merge all liability files into single liability file
4. Run `render` to create phenotypes based on liabilities

#Limitations

All VCF files need to contain the same samples in the same order.
This is what people normally have anyway.

Since results are based on randomly chosen allelic effects, different
runs produce different phenotypes.

#Phenotype definitions

To create a phenotype called `foo` based on normally distributed allele effects
and heritability of 0.3, write:

```
foo=norm(0,1),0.3
```

To create a phenotype where 0.1 of alleles have an effect, 0.01 have an effect
ten times as large, and the remaining 0.89 alleles have no effect, write:

```
foo=pick(0.1,norm(0,1),0.01,norm(0,10),0.89,0)
```

To get ten phenotypes named `foo0` to `foo9`, write:

```
foo[10]=norm(0,1),0.3
```

To create ten binary phenotypes called `color0` to `color9` with possible
values yellow and blue, with a prevalence of 0.47 for yellow, write:

```
color[10]=norm(0,1),0.3,bin(0.47,yellow,blue)
```

#Usage

```
phenics 0.1.0
Oliver Ruebenacker <oliverr@broadinstitute.org>
Phenotype simulator

USAGE:
    phenics [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    check
    help      Print this message or the help of the given subcommand(s)
    merge
    render
    vcf
```

##check

```
USAGE:
    phenics check [OPTIONS]

OPTIONS:
    -h, --help                Print help information
    -p, --phenotype <FILE>    Phenotype definitions file
```

##vcf

```
USAGE:
    phenics vcf [OPTIONS]

OPTIONS:
    -h, --help                Print help information
    -i, --input <FILE>...     Input files (VCF)
    -o, --output <FILE>       Output file
    -p, --phenotype <FILE>    Phenotype definitions file
```

##merge

```
USAGE:
    phenics merge [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -i, --input <FILE>...    Input files (liabilities)
    -o, --output <FILE>      Output file
```

##render

```
USAGE:
    phenics render [OPTIONS]

OPTIONS:
    -h, --help                Print help information
    -i, --input <FILE>...     Input files (liabilities)
    -o, --output <FILE>       Output file
    -p, --phenotype <FILE>    Phenotype definitions file
```

#Credits

Written by Oliver Ruebenacker based on guidance by Jason Flannick and 
Yunfeng Ruan.