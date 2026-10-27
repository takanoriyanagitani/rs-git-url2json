iwasi=./git-url2json.wasm
out="./opt.wasm"

wasm-opt \
	-Oz \
	-o "${out}" \
	--enable-bulk-memory \
	--enable-nontrapping-float-to-int \
	--enable-simd \
	"${iwasi}"
