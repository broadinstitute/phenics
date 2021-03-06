use crate::config::{Config, get_config};
use crate::error::Error;

mod config;
mod error;
mod sim;
mod read;
mod locus;
mod phenotype;
mod vcf;
mod check;
mod merge;
mod render;
mod stats;
mod download;
mod gcs;
mod http;
mod gc_auth;
mod records;
mod region;
mod tabix;
mod region_iter;
mod sample;

pub fn run() -> Result<(), Error> {
    let config = get_config()?;
    match config {
        Config::Check(check_config) => { check::check(&check_config) }
        Config::Vcf(vcf_config) => { vcf::process_vcf(&vcf_config) }
        Config::Merge(merge_config) => { merge::merge(&merge_config) }
        Config::Render(render_config) => { render::render(&render_config) }
        Config::Download(download_config) => { download::download(&download_config) }
        Config::GcsTabix(gcs_tabix_config) => { tabix::tabix(&gcs_tabix_config) }
        Config::GcsSample(gcs_sample_config) => { sample::sample(&gcs_sample_config) }
    }
}