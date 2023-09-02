use wasm_bindgen::prelude::*;
use speedy::{Readable, Writable};

/// data is an `n * k` matrix
/// dims is (n, k)
/// *A Concrete Builder and HNSWInitializer*
#[wasm_bindgen]
#[derive(Clone, Readable, Writable)]
pub struct Graph {
  data: Box<[i32]>,
  eps: Vec<i32>,
  max_edge_count: usize,
  max_node_count: usize,
}

#[wasm_bindgen]
impl Graph {
  #[allow(non_snake_case)]
  #[wasm_bindgen(getter)]
  pub fn EMPTY_ID() -> i32 { -1 }
}

#[wasm_bindgen]
impl Graph {
  #[wasm_bindgen(constructor)]
  pub fn new(
    max_node_count: usize,
    max_edge_count: usize,
  ) -> Graph {
    let mut result = Graph {
      data: Box::from(Vec::new()),
      eps: Vec::new(),
      max_edge_count,
      max_node_count,
    };
    result.init(max_node_count, max_edge_count);
    result
  }

  pub fn init(
    &mut self,
    max_node_count: usize,
    max_edge_count: usize,
  ) -> () {
    self.data = {
      // [TODO] alloc
      let capacity =
        max_node_count
          .checked_mul(max_edge_count)
          .unwrap_or(usize::MAX);
      Box::from(vec![Graph::EMPTY_ID(); capacity])
    };
    self.max_edge_count = max_edge_count;
    self.max_node_count = max_node_count;
  }

  // pub fn init_search(); // [TODO] Pool, Computer

  pub fn deserialize(buffer: &[u8]) -> Result<Graph, JsValue> {
    Graph::read_from_buffer(buffer)
      .map_err(|e| e.to_string().into())
  }

  pub fn serialize(this: &Graph) -> Result<Box<[u8]>, JsValue> {
    this
      .write_to_vec()
      .map_or_else(
        |e| Err(e.to_string().into()),
        |v| Ok(Box::from(v)),
      )
  }
}

#[wasm_bindgen]
impl Graph {
  pub fn edges(&self, node_index: usize) -> Option<Box<[i32]>> {
    let from = node_index * self.max_edge_count;
    let to = from + self.max_edge_count;
    self.data.get(from..to).map(Box::from)
  }

  pub fn at(
    &self,
    node_index: usize,
    edge_index: usize,
  ) -> Option<i32> {
    let at = node_index * self.max_edge_count + edge_index;
    self.data.get(at).copied()
  }

  pub fn set_at(
    &mut self,
    node_index: usize,
    edge_index: usize,
    id: i32,
  ) -> Result<(), JsValue> {
    let at = node_index * self.max_edge_count + edge_index;
    self.data
      .get_mut(at)
      .map_or(
        Err("Index out of bound".into()),
        |target| {
          *target = id;
          Ok(())
        }
      )
  }

  // pub fn prefetch(); // [TODO] alloc
}
