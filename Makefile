install_renoise:
	rm -rf $(RENOISE_PLUGIN_LOCATION)
	cp -r renoise/xyz.yyna.Calcium.xrnx $(RENOISE_PLUGIN_LOCATION)

editor:
	RUST_LOG="editor=info,warn" cargo run -p editor

tin:
	RUST_LOG="tin=trace,tin_drivers_midi=info,info" cargo run -p tin $(SET)
