import * as gfa from "wasm-gfa";
import { memory } from "wasm-gfa/wasm_gfa_bg";

const gfaJson =
["H	VN:Z:1.0",
"S	1	CAAATAAG",
"S	2	A",
"S	3	G",
"S	4	T",
"S	5	C",
"S	6	TTG",
"S	7	A",
"S	8	G",
"S	9	AAATTTTCTGGAGTTCTAT",
"S	10	A",
"S	11	T",
"S	12	ATAT",
"S	13	A",
"S	14	T",
"S	15	CCAACTCTCTG",
"P	x	1+,3+,5+,6+,8+,9+,11+,12+,14+,15+	8M,1M,1M,3M,1M,19M,1M,4M,1M,11M",
"P	y	1+,2+,4+,6+,7+,9+,11+,12+,14+,15+	8M,1M,1M,3M,1M,19M,1M,4M,1M,11M",
"P	z	1+,3+,5+,6+,7+,9+,10+,12+,13+,15+	8M,1M,1M,3M,1M,19M,1M,4M,1M,11M",
"L	1	+	2	+	0M",
"L	1	+	3	+	0M",
"L	2	+	4	+	0M",
"L	2	+	5	+	0M",
"L	3	+	4	+	0M",
"L	3	+	5	+	0M",
"L	4	+	6	+	0M",
"L	5	+	6	+	0M",
"L	6	+	7	+	0M",
"L	6	+	8	+	0M",
"L	7	+	9	+	0M",
"L	8	+	9	+	0M",
"L	9	+	10	+	0M",
"L	9	+	11	+	0M",
"L	10	+	12	+	0M",
"L	11	+	12	+	0M",
"L	12	+	13	+	0M",
"L	12	+	14	+	0M",
"L	13	+	15	+	0M",
 "L	14	+	15	+	0M"];


window.parse_line = gfa.parse_line;
window.gfaJson = gfaJson;
