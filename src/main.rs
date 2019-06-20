#![feature(test)]

use sgx_isa::{Report, Targetinfo};

extern crate test;

#[cfg(not(target_env = "sgx"))]
pub fn get_qe_ti() {
    let _qe_ti = dcap_ql::target_info().unwrap();
}

#[cfg(not(target_env = "sgx"))]
pub fn get_quote() {
    let report = Report {
        ..Default::default()
    };  

    let _quote = dcap_ql::quote(&report);
}

#[cfg(target_env = "sgx")]
pub fn create_report() {
    let ti = Targetinfo {
        ..Default::default()
    };  

    let report = Report::for_target(&ti, &[0; 64]);
}

#[cfg(target_env = "sgx")]
#[cfg(test)]
mod sgx_tests {
    use super::*;
    use test::Bencher;


    #[bench]
    fn bench_create_report(b: &mut Bencher) {
        b.iter(|| create_report())
    }
}

#[cfg(not(target_env = "sgx"))]
#[cfg(test)]
mod non_sgx_tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_get_qe_ti(b: &mut Bencher) {
        b.iter(|| get_qe_ti())
    }

    #[bench]
    fn bench_get_quote(b: &mut Bencher) {
        b.iter(|| get_quote())
    }
}
