DEBUG = true

require("socket")


---@type Client?
local client = nil
local function client_timer_function()
  if client ~= nil then
    client:callback()
  end
end

renoise.tool():add_menu_entry({
	name = "Main Menu:Tools:Connect to Tin",
	active = function() return client == nil end,
	invoke = function()
	  client = Client()
		renoise.tool():add_timer(client_timer_function, 10)
		-- renoise.app():show_prompt("Calcium connected", "Calcium connected", { "OK" })
		-- renoise.song().transport.sync_mode = renoise.Transport.SYNC_MODE_MIDI_CLOCK
	end,
})

renoise.tool():add_menu_entry({
	name = "Main Menu:Tools:Disconnect from Tin",
	active = function() return client ~= nil end,
	invoke = function()
	  if client ~= nil then
	    client:destroy()
		end
	  client = nil
		renoise.tool():remove_timer(client_timer_function)
		-- renoise.app():show_prompt("Calcium disconnected", "Calcium disconnected", { "OK" })
		-- renoise.song().transport.sync_mode = renoise.Transport.SYNC_MODE_INTERNAL
	end,
})
