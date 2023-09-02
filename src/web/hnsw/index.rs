use wasm_bindgen::prelude::*;
use crate::{
  graph::Graph,
  common::Metric,
};

#[wasm_bindgen]
pub struct Index {
  nb: usize,
  dim: usize,
  m: usize,
  efC: usize,
  // std::unique_ptr<hnswlib::HierarchicalNSW<float>> hnsw = nullptr;
  // std::unique_ptr<hnswlib::SpaceInterface<float>> space = nullptr;
  final_graph: Graph,
}

#[wasm_bindgen]
impl Index {
  #[wasm_bindgen(constructor)]
  pub fn new(
    dim: usize,
    metric: Metric,
    r: Option<usize>,
    l: Option<usize>,
  ) {

  }

  // pub fn build() {}

  // #[wasm_bindgen(getter)]
  // pub fn graph() {}
}

