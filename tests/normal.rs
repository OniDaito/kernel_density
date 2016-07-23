mod common;

extern crate kernel_density;
extern crate rand;
extern crate quickcheck;

use kernel_density::kde::normal::NormalKernelDensityEstimation;
use common::{check, SamplesF64, PositiveF64};
use std::f64;

#[test]
#[should_panic(expected="assertion failed: length > 0")]
fn new_normal_kde_panics_on_empty_samples_set() {
    let xs: Vec<f64> = vec![];
    NormalKernelDensityEstimation::new(&xs);
}

#[test]
#[should_panic(expected="assertion failed: bandwidth > 0.0")]
fn normal_kde_panics_on_zero_bandwidth() {
    let xs: Vec<f64> = vec![0.0];
    let kde = NormalKernelDensityEstimation::new(&xs);
    kde.value(0.0, 0.0);
}

#[test]
fn normal_kde_between_zero_and_one() {
    fn prop(xs: SamplesF64, x: f64, bandwidth: PositiveF64) -> bool {
        let kde = NormalKernelDensityEstimation::new(&xs.vec);
        let actual = kde.value(x, bandwidth.val);

        0.0 <= actual && actual <= 1.0
    }

    check(prop as fn(SamplesF64, f64, PositiveF64) -> bool);
}

#[test]
fn normal_kde_cdf_between_zero_and_one() {
    fn prop(xs: SamplesF64, x: f64, bandwidth: PositiveF64) -> bool {
        let kde = NormalKernelDensityEstimation::new(&xs.vec);
        let actual = kde.cdf(x, bandwidth.val);

        0.0 <= actual && actual <= 1.0
    }

    check(prop as fn(SamplesF64, f64, PositiveF64) -> bool);
}

#[test]
fn normal_kde_cdf_is_an_increasing_function() {
    fn prop(xs: SamplesF64, x: f64, bandwidth: PositiveF64) -> bool {
        let kde = NormalKernelDensityEstimation::new(&xs.vec);
        let actual = kde.cdf(x, bandwidth.val);

        kde.cdf(x - 0.01, bandwidth.val) <= actual && actual <= kde.cdf(x + 0.01, bandwidth.val)
    }

    check(prop as fn(SamplesF64, f64, PositiveF64) -> bool);
}
