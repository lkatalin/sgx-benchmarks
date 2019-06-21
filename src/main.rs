#![feature(test)]

use sgx_isa::{Report, Targetinfo};
extern crate test;

// benchmark within an enclave
#[cfg(target_env = "sgx")]
#[cfg(test)]
mod sgx_tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_create_report(b: &mut Bencher) {
        b.iter(|| {
            let ti = Targetinfo::default();
            Report::for_target(&ti, &[0; 64])
        })
    }
}

// benchmark outside of an enclave
#[cfg(not(target_env = "sgx"))]
#[cfg(test)]
mod non_sgx_tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_get_qe_targetinfo(b: &mut Bencher) {
        b.iter(|| dcap_ql::target_info().unwrap())
    }

    #[bench]
    fn bench_get_quote(b: &mut Bencher) {
        b.iter(|| {
            let report = Report::default();
            dcap_ql::quote(&report)
        })
    }
}
