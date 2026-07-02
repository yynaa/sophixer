.PHONY: gen_renoise_massage

gen_renoise_massage:
	mkdir -p renoise/xyz.yyna.Calcium.xrnx/messages
	cd renoise/massage; luajit generator.lua

install_renoise: gen_renoise_massage
	rm -rf $(RENOISE_PLUGIN_LOCATION)
	cp -r renoise/xyz.yyna.Calcium.xrnx $(RENOISE_PLUGIN_LOCATION)

editor:
	RUST_LOG="editor=info,warn" cargo run -p editor

tin:
	RUST_LOG="tin=trace,tin_drivers_midi=info,info" cargo run -p tin $(SET)

tin_debug:
	RUST_LOG="tin=trace,tin_drivers_midi=info,intercom=trace,info" cargo run -p tin $(SET)
