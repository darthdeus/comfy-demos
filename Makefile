BIN=bitmob
FEATURES=comfy/dev

default: desktop
# default: wasm

desktop:
	RUST_LOG=info,naga=warn,wgpu=warn,symphonia=warn RUST_BACKTRACE=1 cargo run --bin bitmob --features $(FEATURES)

wasm:
	cd $(BIN) && trunk build --features $(FEATURES) --release

serve:
	cd $(BIN)/dist && python -m http.server

bitmob-1bit-jam:
	git pull
	git branch -f bitmob-1bit-jam HEAD
	git push origin bitmob-1bit-jam

