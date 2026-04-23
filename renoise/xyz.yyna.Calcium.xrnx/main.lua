require("utils")

require("playback_control")

require("socket")

loading_song = false

---@type Client?
local client = nil
local function client_timer_function()
  if client ~= nil then
    client:callback()
  end
end

renoise.tool():add_menu_entry({
	name = "Main Menu:Tools:Calcium:Connect",
	active = function() return client == nil end,
	invoke = function()
	  client = Client()
		renoise.tool():add_timer(client_timer_function, 10)
		-- renoise.app():show_prompt("Calcium connected", "Calcium connected", { "OK" })
	end,
})

renoise.tool():add_menu_entry({
	name = "Main Menu:Tools:Calcium:Disconnect",
	active = function() return client ~= nil end,
	invoke = function()
	  if client ~= nil then
	    client:destroy()
		end
	  client = nil
		renoise.tool():remove_timer(client_timer_function)
		renoise.app():show_prompt("Calcium disconnected", "Calcium disconnected", { "OK" })
	end,
})

renoise.tool().app_new_document_observable:add_notifier(function()
  if loading_song then --avoids multiple calls
    print("song loaded")
    loading_song = false
  end
end)
