version 1.0

workflow phenics {
    input {
        Array[File] vcf_files
        File phenotypes_file
        String output_file_name
    }
    scatter(vcf_file in vcf_files) {
        call process_vcf {
            input:
                vcf_file = vcf_file,
                phenotypes_file = phenotypes_file,
                output_file_name = "liabilities"
        }
    }
    call merge {
        input:
            liabilities_files = process_vcf.output_file,
            phenotypes_file = phenotypes_file,
            output_file_name = output_file_name
    }
}

task process_vcf {
    input {
        File vcf_file
        File phenotypes_file
        String output_file_name
    }
    parameter_meta {
        vcf_file: {
            description: "a VCF file",
            localization_optional: true
        }
    }
    runtime {
        docker: "gcr.io/nitrogenase-docker/phenics:0.2.11"
        memory: "16 GB"
        disks: "local-disk 80 HDD"
    }
    command <<<
        set -e
        echo "Now running phenics sample for ~{vcf_file}"
        phenics sample -d ~{vcf_file} -p ~{phenotypes_file} -r 1000 -x 10000000 -o ~{output_file_name}
    >>>
    output {
        File output_file = output_file_name
    }
}

task merge {
    input {
        Array[File] liabilities_files
        File phenotypes_file
        String output_file_name
    }
    runtime {
        docker: "gcr.io/nitrogenase-docker/phenics:0.2.11"
        memory: "16 GB"
        disks: "local-disk 80 HDD"
    }
    command <<<
        set -e
        phenics merge -i ~{liabilities_files} -o merged
        phenics render -i merged -p ~{phenotypes_file} -o ~{output_file_name}
    >>>
    output {
        File output_file = output_file_name
    }
}