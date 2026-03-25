require("utils")
require("socket")

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
