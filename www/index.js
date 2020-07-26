import * as gfa from "wasm-gfa";
import { memory } from "wasm-gfa/wasm_gfa_bg";

const ptrBytes = ({start, len}) => {
  let bytes = new Uint8Array(memory.buffer, start, len);
  return bytes;
};

const ptrString = ({start, len}) => {
  let bytes = new Uint8Array(memory.buffer, start, len);
  let string = new TextDecoder('utf8').decode(bytes);
  return string;
}


const get_segment = (graph, ix) => {
  let nPtr = graph.get_segment_name(ix);
  let sPtr = graph.get_segment_seq(ix);
  let name = ptrString(nPtr);
  let seq = ptrString(sPtr);
  return { name, seq };
}

const unpack_string_ptr = (ptr) => {
  let string_arr = new Uint32Array(memory.buffer, ptr, 3);
  return { start: string_arr[0],
           len: string_arr[1],
           cap: string_arr[2] }
};

const unpack_segments = (ptr, len) => {
  let result = [];
  for (let i = 0; i < len; i++) {
    let name_ptr = ptr + i * 24;
    let seq_ptr = ptr + 12 + i * 24;

    let name_str = unpack_string_ptr(name_ptr);
    let seq_str = unpack_string_ptr(seq_ptr);

    let name = ptrString(name_str);
    let seq = ptrString(seq_str);

    result.push({name, seq});
  }

  return result;
}

const seg_from_memory = (ptr) => {
  let name_ptr = ptr;
  let seq_ptr = ptr + 12; // Rust string size in wasm

};

window.ptrBytes = ptrBytes;
window.ptrString = ptrString;


const main = (graph) => {
  window.graph = graph;

  window.getSegment = (ix) => {
    return get_segment(graph, ix);
  };

  console.log("segments start at " + graph.segments());
  console.log("jsegment size: " + gfa.seg_size());
  console.log("string size: " + gfa.string_size());


  let segments_name_ptr = graph.segments();
  let segments_seq_ptr = segments_name_ptr + gfa.string_size();

  let seq_string = unpack_string_ptr(segments_seq_ptr);

  console.log("Seq parts: ");
  console.log(seq_string.ptr + " - " + seq_string.len);

  let { name, seq } = get_segment(graph, 0);
  console.log("  " + name + " - " + seq);

  let segments = unpack_segments(graph.segments(), 5);
  console.log(segments);
  window.segments = segments;

};

gfa.fetch_wgfa("./lil.gfa").then(g => main(g));
