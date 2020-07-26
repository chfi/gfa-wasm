import * as gfa from "wasm-gfa";
import { memory } from "wasm-gfa/wasm_gfa_bg";


const ptrBytes = ({start, len}) => {
  let bytes = new Uint8Array(memory.buffer, start, len);
  return bytes;
};

const rename_segment = (namePtr, text) => {
  let { start, len } = namePtr;
  let mem_bytes = new Uint8Array(memory.buffer, start, len);
  let text_bytes = new TextEncoder('utf8').encode(text);
  mem_bytes.set(text_bytes, 0);
};

window.renameSeg = rename_segment;

const ptrString = ({start, len}) => {
  let bytes = new Uint8Array(memory.buffer, start, len);
  let string = new TextDecoder('utf8').decode(bytes);
  console.log("before: " + string);
  return string;
};


const get_segment_ptrs = (graph, ix) =>{
  let name = graph.get_segment_name(ix);
  let seq = graph.get_segment_seq(ix);
  return { name, seq };
};

const get_segment = (graph, ix) => {
  let nPtr = graph.get_segment_name(ix);
  let sPtr = graph.get_segment_seq(ix);
  let name = ptrString(nPtr);
  let seq = ptrString(sPtr);
  return { name, seq };
};

const unpack_string_ptr = (ptr) => {
  let string_arr = new Uint32Array(memory.buffer, ptr, 3);
  return { start: string_arr[0],
           len: string_arr[1],
           cap: string_arr[2] }
};

const slice_callback = (slice, cb) => {
  for (let i = 0; i < slice.len; i++) {
    let ptr = slice.start + i * slice.stride;
    let bytes = new Uint8Array(memory.buffer, ptr, slice.stride);
    cb(bytes)
  }
};

window.sliceCB = slice_callback;

const slice_collect_bytes = (slice) => {
  let result = [];
  for (let i = 0; i < slice.len; i++) {
    let ptr = slice.start + i * slice.stride;
    let bytes = new Uint8Array(memory.buffer, ptr, slice.stride);
    result.push(bytes);
  }
  return result;
};

window.sliceCollect = slice_collect_bytes;

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
};
window.gfa = gfa;

gfa.fetch_gfa("./lil.gfa").then(g => main(g));

window.gfamem = memory;
