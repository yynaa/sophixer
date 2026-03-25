RENOISE_PLUGIN_LOCATION=/mnt/Programs/Wine/Music/drive_c/users/yyna/AppData/Roaming/Renoise/V3.5.3/Scripts/Tools/xyz.yyna.Calcium.xrnx

install_renoise:
	rm -rf $(RENOISE_PLUGIN_LOCATION)
	cp -r renoise/xyz.yyna.Calcium.xrnx $(RENOISE_PLUGIN_LOCATION)
