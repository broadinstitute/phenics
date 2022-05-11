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
    runtime {
        docker: "gcr.io/nitrogenase-docker/phenics:0.1.0"
        memory: "16 GB"
        disks: "local-disk 80 HDD"

    }
    command <<<
        set -e
        phenics vcf -i ~{vcf_file} -p ~{phenotypes_file} -o ~{output_file_name}
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
        docker: "gcr.io/nitrogenase-docker/phenics:0.1.0"
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