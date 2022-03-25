//! Functions to improve UX
use indicatif::{ProgressBar, ProgressStyle};

pub fn progress_bar(len: u64) -> ProgressBar {
    let bar = ProgressBar::new(len);
    let template = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}]  {pos:>7}/{len:7}  ({eta_precise})";
    bar.set_style(
        ProgressStyle::default_bar()
            .template(template)
            .progress_chars("=> "),
    );

    bar
}
